use anyhow::Result;
use std::path::PathBuf;
use storage_api_test::{discovery::DiscoveryApi, generator::Generator};
use tempfile::TempDir;

#[tokio::test]
async fn test_generate_storage_api() -> Result<()> {
    // Set up temporary directory
    let temp_dir = TempDir::new()?;
    let output_dir = temp_dir.path().to_path_buf();

    // Get API definition
    let discovery = DiscoveryApi::new();
    println!("Created discovery client: {:#?}", discovery);

    let api_def = discovery.get_storage_api("v1").await?;
    println!("Got API definition:");
    api_def.print_debug(); // Using the new helper method

    // Initialize generator
    let generator = Generator::new(PathBuf::from("templates"), output_dir.clone(), api_def)?;

    // Generate code
    generator.generate()?;

    // Verify generated files exist
    assert!(output_dir.join("storagev1/Cargo.toml").exists());
    assert!(output_dir.join("storagev1/src/lib.rs").exists());
    assert!(output_dir.join("storagev1/src/client.rs").exists());
    assert!(output_dir.join("storagev1/src/types.rs").exists());
    assert!(output_dir.join("storagev1-cli/Cargo.toml").exists());
    assert!(output_dir.join("storagev1-cli/src/main.rs").exists());

    Ok(())
}
