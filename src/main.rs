use clap::Parser;

use minecraft_download::{download_version, fetch_manifest};
use minecraft_download::cli::Args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let versions_dir = args.out_dir.join("versions");
    let assets_dir = args.out_dir.join("assets");

    println!("Fetching version manifest...");
    let manifest = fetch_manifest().await?;
    let latest = &manifest.versions[0];

    download_version(latest, &versions_dir).await?;

    Ok(())
}
