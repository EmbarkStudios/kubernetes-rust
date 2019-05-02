use std::sync::Arc;

use failure::Error;
use futures::compat::Future01CompatExt;
use http;
use serde::de::DeserializeOwned;

use super::config::Configuration;

/// APIClient requires `config::Configuration` includes client to connect with kubernetes cluster.
pub struct APIClient {
    configuration: Arc<Configuration>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> Self {
        let rc = Arc::new(configuration);
        APIClient { configuration: rc }
    }

    /// Returns kubernetes resources binded `Arnavion/k8s-openapi-codegen` APIs.
    pub async fn request<T>(&self, request: http::Request<Vec<u8>>) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let (parts, body) = request.into_parts();
        let uri_str = format!("{}{}", self.configuration.base_path, parts.uri);
        let mut res = await!(match parts.method {
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

        await!(res.json().compat()).map_err(Error::from)
    }
}
