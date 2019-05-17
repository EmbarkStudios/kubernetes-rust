use std::{fmt, sync::Arc};

use failure::Error;
use futures::{
    compat::Stream01CompatExt,
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
        let res = self.configuration.client(request).await?;

        let mut json_body = Vec::with_capacity(res.content_length().unwrap_or(1024) as usize);

        // If an API error can't be deserialized from the error response's JSON body, use
        // the HTTP error as a fallback
        let fallback_err = res.error_for_status_ref().map(|_| ());
        let mut res_body = res.into_body().compat();

        while let Some(chunk) = res_body.next().await {
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
