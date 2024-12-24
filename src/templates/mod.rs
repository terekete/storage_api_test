mod client;
mod error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateData {
    pub name: String,
    pub version: String,
    pub revision: String,
    pub title: String,
    pub description: String,
    pub documentation: String,
    pub schemas: serde_json::Value,
    pub methods: serde_json::Value,
}
