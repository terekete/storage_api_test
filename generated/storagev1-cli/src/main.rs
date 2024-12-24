use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use google_storagev1::prelude::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Configuration directory
    #[arg(long, default_value = "~/.google-service-cli")]
    config_dir: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up authentication
    let secret = yup_oauth2::read_application_secret(cli.config_dir.join("client_secret.json"))
        .await
        .context("Failed to read client secret")?;

    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(secret)
        .persist_tokens_to_disk(cli.config_dir.join("tokens.json"))
        .build()
        .await?;

    let client = StorageClient::new(auth);

    match cli.command {
        
    }

    Ok(())
}