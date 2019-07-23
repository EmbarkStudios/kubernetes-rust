mod apis;
mod incluster_config;
mod kube_config;
mod utils;

use base64;
use failure::Error;
use futures::{
    compat::{Future01CompatExt, Stream01CompatExt},
    stream::StreamExt,
};
use oauth::gcp::{ServiceAccountInfo, ServiceAccountAccess, TokenOrRequest};
use reqwest::{
    header,
    r#async::{Client, Response},
    Certificate, Identity,
};
use std::sync::Arc;

use self::kube_config::KubeConfigLoader;

/// Configuration stores kubernetes path and client for requests.
pub struct Configuration {
    base_path: String,
    client: Client,
    auth_provider: Option<AuthProvider>,
}

impl Configuration {
    pub(crate) fn new(
        base_path: String,
        client: Client,
        auth_provider: Option<AuthProvider>,
    ) -> Self {
        Configuration {
            base_path,
            client,
            auth_provider,
        }
    }

    pub(crate) async fn client(
        &self,
        mut request: http::Request<Vec<u8>>,
    ) -> Result<Response, Error> {
        let client = self.client.clone();

        if let Some(ref auth_provider) = self.auth_provider {
            let auth_value = await!(auth_provider.get_auth_header(&client))?;

            request
                .headers_mut()
                .insert(header::AUTHORIZATION, auth_value);
        }

        let (parts, body) = request.into_parts();
        let uri_str = format!("{}{}", self.base_path, parts.uri);

        let send = async move {
            let req_builder = match parts.method {
                http::Method::GET => client.get(&uri_str),
                http::Method::POST => client.post(&uri_str),
                http::Method::DELETE => client.delete(&uri_str),
                http::Method::PUT => client.put(&uri_str),
                other => {
                    return Err(Error::from(format_err!("Invalid method: {}", other)));
                }
            };

            let req = req_builder.headers(parts.headers).body(body);

            Ok(await!(req.send().compat())?)
        };

        await!(send)
    }
}

pub(crate) enum AuthProvider {
    //Basic(header::HeaderValue),
    Oauth2(Arc<ServiceAccountAccess>),
}

impl AuthProvider {
    // fn with_username_password(username: &str, password: &str) -> Result<AuthProvider, Error> {
    //     let encoded = base64::encode(&format!("{}:{}", username, password));
    //     let hv = header::HeaderValue::from_str(&format!("Basic {}", encoded))?;

    //     Ok(AuthProvider::Basic(hv))
    // }

    fn with_service_key(key: ServiceAccountInfo) -> Result<AuthProvider, Error> {
        let access = ServiceAccountAccess::new(key)?;
        Ok(AuthProvider::Oauth2(Arc::new(access)))
    }

    async fn get_auth_header<'a>(
        &'a self,
        client: &'a Client,
    ) -> Result<header::HeaderValue, Error> {
        match self {
            //AuthProvider::Basic(hv) => Ok(hv.clone()),
            AuthProvider::Oauth2(access) => {
                let token = match access
                    .get_token(&["https://www.googleapis.com/auth/cloud-platform"])
                    .map_err(|e| format_err!("failed to request token: {}", e))?
                {
                    TokenOrRequest::Token(token) => token,
                    TokenOrRequest::Request {
                        request,
                        scope_hash,
                        ..
                    } => {
                        let (parts, body) = request.into_parts();

                        let response: Result<http::Response<_>, Error> = await!(async {
                            let req_builder = match parts.method {
                                // Should only ever be POST...
                                http::Method::POST => client.post(&format!("{}", parts.uri)),
                                method => {
                                    unreachable!("this...should not have happened: {}", method)
                                }
                            };

                            let response = await!(req_builder
                                .headers(parts.headers)
                                .body(body)
                                .send()
                                .compat())?;

                            // The oauth code only really cares about the status and body
                            // so the rest being empty is fine
                            let status = response.status();

                            let mut json_body = Vec::with_capacity(
                                response.content_length().unwrap_or(1024) as usize,
                            );
                            let mut res_body = response.into_body().compat();

                            while let Some(chunk) = await!(res_body.next()) {
                                let chunk = chunk?;
                                json_body.extend_from_slice(&chunk[..])
                            }

                            let response =
                                http::Response::builder().status(status).body(json_body)?;

                            Ok(response)
                        });

                        access
                            .parse_token_response(scope_hash, response?)
                            .map_err(|e| format_err!("failed to acquire token: {}", e))?
                    }
                };

                Ok(header::HeaderValue::from_str(&format!(
                    "Bearer {}",
                    token.access_token
                ))?)
            }
        }
    }
}

/// Returns a config includes authentication and cluster information from kubeconfig file.
///
/// # Example
/// ```no_run
/// use kubernetes::config;
///
/// let kubeconfig = config::load_kube_config()
///     .expect("failed to load kubeconfig");
/// ```
pub fn load_kube_config() -> Result<Configuration, Error> {
    let kubeconfig = utils::kubeconfig_path()
        .or_else(utils::default_kube_path)
        .ok_or(format_err!("Unable to load kubeconfig"))?;

    let loader = KubeConfigLoader::load(kubeconfig)?;
    let mut client_builder = Client::builder();

    if let Some(ca) = loader.ca() {
        let req_ca = Certificate::from_der(&ca?.to_der()?)?;
        client_builder = client_builder.add_root_certificate(req_ca);
    }
    match loader.p12(" ") {
        Ok(p12) => {
            let req_p12 = Identity::from_pkcs12_der(&p12.to_der()?, " ")?;
            client_builder = client_builder.identity(req_p12);
        }
        Err(_) => {
            // last resort only if configs ask for it, and no client certs
            if let Some(true) = loader.cluster.insecure_skip_tls_verify {
                client_builder = client_builder.danger_accept_invalid_certs(true);
            }
        }
    }

    let auth_provider = match (
        utils::data_or_file(&loader.user.token, &loader.user.token_file),
        (loader.user.username, loader.user.password),
    ) {
        (Ok(_), _) => {
            let path = std::env::var_os("GOOGLE_APPLICATION_CREDENTIALS")
                .map(std::path::PathBuf::from)
                .ok_or(format_err!("Missing GOOGLE_APPLICATION_CREDENTIALS env",))?;

            let svc_acct_info = std::fs::read_to_string(path)?;

            Some(AuthProvider::with_service_key(
                ServiceAccountInfo::deserialize(svc_acct_info)?,
            )?)
        }
        (_, (Some(u), Some(p))) => {
            let mut headers = header::HeaderMap::new();

            let encoded = base64::encode(&format!("{}:{}", u, p));
            let hv = header::HeaderValue::from_str(&format!("Basic {}", encoded))?;

            headers.insert(header::AUTHORIZATION, hv);

            client_builder = client_builder.default_headers(headers);

            None
        }
        _ => return Err(format_err!("unable to find an auth-provider")),
    };

    Ok(Configuration::new(
        loader.cluster.server,
        client_builder.build()?,
        auth_provider,
    ))
}

/// Returns a config which is used by clients within pods on kubernetes.
/// It will return an error if called from out of kubernetes cluster.
///
/// # Example
/// ```no_run
/// use kubernetes::config;
///
/// let kubeconfig = config::incluster_config()
///     .expect("failed to load incluster config");
/// ```
pub fn incluster_config() -> Result<Configuration, Error> {
    let server = incluster_config::kube_server().ok_or(format_err!(
        "Unable to load incluster config, {} and {} must be defined",
        incluster_config::SERVICE_HOSTENV,
        incluster_config::SERVICE_PORTENV
    ))?;

    let ca = incluster_config::load_cert()?;
    let req_ca = Certificate::from_der(&ca.to_der()?)?;

    let token = incluster_config::load_token()?;
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", token))?,
    );

    let client_builder = Client::builder()
        .add_root_certificate(req_ca)
        .default_headers(headers);

    Ok(Configuration::new(server, client_builder.build()?, None))
}
