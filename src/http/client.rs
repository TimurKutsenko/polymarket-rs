use crate::error::{Error, Result};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;

/// HTTP client wrapper for making API requests
#[derive(Clone)]
pub struct HttpClient {
    client: Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONNECTION, CONTENT_TYPE, USER_AGENT};

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("py_clob_client"));
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
        headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        Self {
            client: Client::builder()
                .default_headers(headers)
                .build()
                .expect("Failed to create HTTP client"),
            base_url: base_url.into(),
        }
    }

    /// Make a GET request
    pub async fn get<T>(&self, path: &str, headers: Option<HashMap<&str, String>>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.get(&url);

        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Make a POST request with JSON body
    pub async fn post<T, B>(
        &self,
        path: &str,
        body: &B,
        headers: Option<HashMap<&str, String>>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.post(&url).json(body);

        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Make a DELETE request with optional JSON body
    pub async fn delete<T>(&self, path: &str, headers: Option<HashMap<&str, String>>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.delete(&url);

        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Make a DELETE request with JSON body
    pub async fn delete_with_body<T, B>(
        &self,
        path: &str,
        body: &B,
        headers: Option<HashMap<&str, String>>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.delete(&url).json(body);

        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Handle response and parse JSON or return error
    async fn handle_response<T>(&self, response: Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let status = response.status();

        if status.is_success() {
            response.json().await.map_err(|e| e.into())
        } else {
            let message = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            Err(Error::Api {
                status: status.as_u16(),
                message,
            })
        }
    }
}
