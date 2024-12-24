use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const STORAGE_DISCOVERY_URL: &str = "https://storage.googleapis.com/$discovery/rest";

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiDefinition {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub revision: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub documentation: Option<String>,
    #[serde(default)]
    pub schemas: serde_json::Value,
    #[serde(default)]
    pub resources: serde_json::Value,
    #[serde(default)]
    pub methods: Option<serde_json::Value>,
    #[serde(rename = "baseUrl", default = "default_base_url")]
    pub base_url: String,
    #[serde(rename = "rootUrl", default = "default_root_url")]
    pub root_url: String,
    #[serde(rename = "servicePath", default)]
    pub service_path: String,
    #[serde(default)]
    pub parameters: Option<serde_json::Value>,
    #[serde(default)]
    pub auth: Option<serde_json::Value>,
}

fn default_base_url() -> String {
    "https://storage.googleapis.com/storage/v1".to_string()
}

fn default_root_url() -> String {
    "https://storage.googleapis.com/".to_string()
}

#[derive(Debug)]
pub struct DiscoveryApi {
    client: Client,
}

impl DiscoveryApi {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_storage_api(&self, version: &str) -> Result<ApiDefinition> {
        let url = format!("{}?version={}", STORAGE_DISCOVERY_URL, version);

        println!("Fetching Storage API definition from: {}", url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch Storage API definition")?;

        let status = response.status();
        println!("Response status: {}", status);

        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!(
                "Failed to fetch API definition: {} - {}",
                status,
                error_text
            ));
        }

        // Get the raw response text and parse it to examine the structure
        let response_text = response.text().await?;
        let raw_json: serde_json::Value = serde_json::from_str(&response_text)?;

        // Debug print schema structure
        if let Some(schemas) = raw_json.get("schemas") {
            println!("\nSchema Structure Example:");
            if let Some(schema_obj) = schemas.as_object() {
                if let Some((first_schema_name, first_schema)) = schema_obj.iter().next() {
                    println!("First schema '{}' structure:", first_schema_name);
                    println!("{}", serde_json::to_string_pretty(first_schema)?);
                }
            }
        }

        // Parse into our ApiDefinition
        let definition: ApiDefinition = serde_json::from_str(&response_text)?;
        Ok(definition)
    }
}

impl ApiDefinition {
    pub fn print_debug(&self) {
        println!("\n=== Storage API Definition ===");
        println!("Name: {}", self.name);
        println!("Version: {}", self.version);
        println!("Title: {}", self.title);
        println!("Description: {}", self.description);
        println!("Base URL: {}", self.base_url);

        println!("\nSchemas:");
        if let Some(schemas) = self.schemas.as_object() {
            for (name, schema) in schemas {
                println!("\nSchema: {}", name);
                println!("Raw schema structure:");
                println!(
                    "{}",
                    serde_json::to_string_pretty(&schema).unwrap_or_default()
                );
            }
        }
    }
}
