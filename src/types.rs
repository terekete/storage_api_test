use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Buckets {
    pub kind: String,
    pub items: Vec<Bucket>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bucket {
    pub kind: String,
    pub id: String,
    pub name: String,
    pub project_number: String,
    pub location: String,
    pub storage_class: String,
    pub time_created: String,
    pub updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Objects {
    pub kind: String,
    pub items: Vec<Object>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    pub kind: String,
    pub id: String,
    pub name: String,
    pub bucket: String,
    pub generation: String,
    pub metageneration: String,
    pub content_type: String,
    pub time_created: String,
    pub updated: String,
    pub storage_class: String,
    pub size: String,
    pub md5_hash: String,
    pub media_link: String,
    pub self_link: String,
}
