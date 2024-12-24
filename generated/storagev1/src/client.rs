use std::sync::Arc;
use hyper::{Body, Client, Request, Method, Uri, service::Service};
use hyper_rustls::HttpsConnectorBuilder;
use serde::de::DeserializeOwned;
use yup_oauth2::authenticator::Authenticator;
use anyhow::Result;

pub struct StorageClient<S>
where
    S: Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    client: Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    auth: Arc<Authenticator<S>>,
}

impl<S> StorageClient<S>
where
    S: Service<Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    pub fn new(auth: Authenticator<S>) -> Self {
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

    async fn request<T: DeserializeOwned>(&self, method: Method, url: String) -> Result<T> {
        let token = self.auth.token(&["https://www.googleapis.com/auth/devstorage.full_control"])
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
            return Err(anyhow::anyhow!("Request failed: {} - {}", status, error_text));
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
        self.request(Method::GET, url).await
    }

    pub async fn bucket_get(&self, bucket_name: &str) -> Result<Bucket> {
        let url = format!(
            "https://storage.googleapis.com/storage/v1/b/{}",
            bucket_name
        );
        self.request(Method::GET, url).await
    }

    pub async fn objects_list(&self, bucket_name: &str) -> Result<Objects> {
        let url = format!(
            "https://storage.googleapis.com/storage/v1/b/{}/o",
            bucket_name
        );
        self.request(Method::GET, url).await
    }

    pub async fn object_get(&self, bucket_name: &str, object_name: &str) -> Result<Object> {
        let url = format!(
            "https://storage.googleapis.com/storage/v1/b/{}/o/{}",
            bucket_name,
            object_name
        );
        self.request(Method::GET, url).await
    }
}

// Type definitions (assuming these are defined elsewhere in your codebase)
pub struct Buckets;
pub struct Bucket;
pub struct Objects;
pub struct Object;