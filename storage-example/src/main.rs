use anyhow::Result;
use google_storagev1::StorageClient;
use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};

#[tokio::main]
async fn main() -> Result<()> {
    // Set up authentication
    let secret = yup_oauth2::read_application_secret("client_secret.json").await?;

    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk("storage_tokens.json")
        .build()
        .await?;

    // Create storage client
    let client = StorageClient::new(auth);

    // List buckets in your project
    let project_id = "your-project-id"; // Replace with your GCP project ID

    println!("Listing buckets in project {}", project_id);

    let buckets = client.buckets_list(project_id).await?;

    // Handle optional values
    if let Some(items) = buckets.items {
        for bucket in items {
            println!("Bucket: {}", bucket.name.unwrap_or_default());
            if let Some(location) = bucket.location {
                println!("  Location: {}", location);
            }
            if let Some(storage_class) = bucket.storage_class {
                println!("  Storage Class: {}", storage_class);
            }

            // List objects in this bucket
            if let Some(bucket_name) = &bucket.name {
                println!("\n  Objects in bucket {}:", bucket_name);
                match client.objects_list(bucket_name).await {
                    Ok(objects) => {
                        if let Some(items) = objects.items {
                            for object in items {
                                println!("    - {}", object.name.unwrap_or_default());
                            }
                        }
                    }
                    Err(e) => println!("    Error listing objects: {}", e),
                }
            }
            println!();
        }
    } else {
        println!("No buckets found");
    }

    Ok(())
}
