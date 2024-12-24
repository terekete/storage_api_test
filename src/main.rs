use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod discovery;
mod generator;
mod templates;

use discovery::DiscoveryApi;
use generator::Generator;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(disable_version_flag = true)] // Disable auto-generated version flag
struct Args {
    /// Output directory for generated code
    #[arg(short, long, default_value = "generated")]
    output_dir: PathBuf,

    /// Templates directory
    #[arg(short, long, default_value = "templates")]
    template_dir: PathBuf,

    /// API version (e.g., v1)
    #[arg(short = 'V', long, default_value = "v1")] // Changed from 'v' to 'V'
    api_version: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("Generating Storage API client version {}", args.api_version);

    let discovery = DiscoveryApi::new();
    let api_def = discovery.get_storage_api(&args.api_version).await?;

    let generator = Generator::new(args.template_dir, args.output_dir, api_def)?;

    generator.generate()?;

    println!("Code generation completed successfully!");
    Ok(())
}
