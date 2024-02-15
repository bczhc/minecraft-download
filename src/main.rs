use minecraft_download::VersionManifest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let res = reqwest::get("https://piston-meta.mojang.com/mc/game/version_manifest.json").await?;
    let manifest = res.json::<VersionManifest>().await?;
    println!("{:?}", manifest);

    Ok(())
}
