use std::{fmt, sync::Arc};

use failure::Error;
use futures::{
    compat::{
        Future01CompatExt,
        Stream01CompatExt,
    },
    stream::StreamExt,
};
use http;
use serde::de::DeserializeOwned;

use super::config::Configuration;

#[derive(Deserialize, Debug, Fail)]
pub struct ApiError {
    status: String,
    #[serde(default)]
    message: Option<String>,
    #[serde(default)]
    reason: Option<String>,
    code: u16,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

/// APIClient requires `config::Configuration` includes client to connect with kubernetes cluster.
#[derive(Clone)]
pub struct APIClient {
    configuration: Arc<Configuration>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> Self {
        APIClient { configuration: Arc::new(configuration) }
    }

    /// Returns kubernetes resources binded `Arnavion/k8s-openapi-codegen` APIs.
    pub async fn request<T>(&self, request: http::Request<Vec<u8>>) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let (parts, body) = request.into_parts();
        let uri_str = format!("{}{}", self.configuration.base_path, parts.uri);
        let res = await!(match parts.method {
            http::Method::GET => self.configuration.client.get(&uri_str),
            http::Method::POST => self.configuration.client.post(&uri_str),
            http::Method::DELETE => self.configuration.client.delete(&uri_str),
            http::Method::PUT => self.configuration.client.put(&uri_str),
            other => {
                return Err(Error::from(format_err!("Invalid method: {}", other)));
            }
        }
        .body(body)
        .send()
        .compat())?;

        let mut json_body = Vec::with_capacity(res.content_length().unwrap_or(1024) as usize);

        // If an API can't be deserialized from the error response's JSON body, use
        // the HTTP error as a fallback
        let fallback_err = res.error_for_status_ref().map(|_| ());
        let mut res_body = res.into_body().compat();

        while let Some(chunk) = await!(res_body.next()) {
            let chunk = chunk?;
            json_body.extend_from_slice(&chunk[..])
        }

        match fallback_err {
            Ok(_) => {
                serde_json::from_slice(&json_body).map_err(|e| {
                    Error::from(e)
                })
            }
            Err(e) => {
                match serde_json::from_slice::<ApiError>(&json_body) {
                    Ok(api_err) => {
                        Err(api_err.into())
                    }
                    Err(_) => {
                        Err(e.into())
                    }
                }
            }
        }
    }
}
