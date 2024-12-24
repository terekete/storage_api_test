use anyhow::Result;
use yup_oauth2::authenticator::Authenticator;

#[tokio::main]
async fn main() -> Result<()> {
    // Set up authentication
    let secret = yup_oauth2::read_application_secret("client_secret.json").await?;
    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(secret)
        .persist_tokens_to_disk("tokens.json")
        .build()
        .await?;

    // Create the client
    let client = StorageClient::new(auth);

    // List buckets in a project
    let buckets = client.buckets_list("your-project-id").await?;
    println!("Buckets: {:?}", buckets);

    // Get a specific bucket
    let bucket = client.bucket_get("your-bucket-name").await?;
    println!("Bucket: {:?}", bucket);

    // List objects in a bucket
    let objects = client.objects_list("your-bucket-name").await?;
    println!("Objects: {:?}", objects);

    // Get a specific object
    let object = client
        .object_get("your-bucket-name", "your-object-name")
        .await?;
    println!("Object: {:?}", object);

    Ok(())
}
