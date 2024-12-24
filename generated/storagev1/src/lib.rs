//! Generated code for storage API version v1
//! Revision: 20241214


//! Stores and retrieves potentially large, immutable data objects.


pub mod types;
pub mod client;

pub use client::StorageClient;
pub use types::*;

pub const API_VERSION: &str = "v1";

pub const BASE_URL: &str = "https://storage.googleapis.com/storage/v1";




pub mod prelude {
    pub use super::client::StorageClient;
    pub use super::types::*;
}