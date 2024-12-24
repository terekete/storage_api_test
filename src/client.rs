use crate::types::*;
use anyhow::Result;
use hyper::{Body, Client, Method, Request};
use hyper_rustls::HttpsConnectorBuilder;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use yup_oauth2::authenticator::Authenticator;

pub struct StorageClient {
    client: Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    auth: Arc<Authenticator>,
}

impl StorageClient {
    pub fn new(auth: Authenticator) -> Self {
        let https = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();

        let client = Client::builder().build(https);

        Self {
            client,
            auth: Arc::new(auth),
        }
    }

    async fn request<T>(&self, method: Method, url: String) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let token = self
            .auth
            .token(&["https://www.googleapis.com/auth/devstorage.full_control"])
            .await?;

        let request = Request::builder()
            .method(method)
            .uri(url)
            .header("Authorization", format!("Bearer {}", token.as_str()))
            .header("Content-Type", "application/json")
            .body(Body::empty())?;

        let response = self.client.request(request).await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = hyper::body::to_bytes(response.into_body()).await?;
            let error_text = String::from_utf8_lossy(&body);
            return Err(anyhow::anyhow!(
                "Request failed: {} - {}",
                status,
                error_text
            ));
        }

        let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
        let result = serde_json::from_slice(&body_bytes)?;
        Ok(result)
    }

    pub async fn buckets_list(&self, project: &str) -> Result<Buckets> {
        let url = format!(
            "https://storage.googleapis.com/storage/v1/b?project={}",
            project
        );
        self.request::<Buckets>(Method::GET, url).await
    }

    pub async fn bucket_get(&self, bucket_name: &str) -> Result<Bucket> {
        let url = format!(
            "https://storage.googleapis.com/storage/v1/b/{}",
            bucket_name
        );
        self.request::<Bucket>(Method::GET, url).await
    }

    pub async fn objects_list(&self, bucket_name: &str) -> Result<Objects> {
        let url = format!(
            "https://storage.googleapis.com/storage/v1/b/{}/o",
            bucket_name
        );
        self.request::<Objects>(Method::GET, url).await
    }

    pub async fn object_get(&self, bucket_name: &str, object_name: &str) -> Result<Object> {
        let url = format!(
            "https://storage.googleapis.com/storage/v1/b/{}/o/{}",
            bucket_name, object_name
        );
        self.request::<Object>(Method::GET, url).await
    }
}
