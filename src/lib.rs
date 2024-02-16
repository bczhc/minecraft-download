use std::{fs, io};
use std::fs::File;
use std::io::Cursor;
use std::path::Path;

use serde::Deserialize;
use sha1::digest::generic_array::GenericArray;

pub mod cli;

#[derive(Deserialize, Debug)]
pub struct LatestVersion {
    pub release: String,
    pub snapshot: String,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub id: String,
    pub r#type: String,
    pub url: String,
    pub time: String,
    pub release_time: String,
}

#[derive(Deserialize, Debug)]
pub struct VersionManifest {
    pub latest: LatestVersion,
    pub versions: Vec<Version>,
}

#[derive(Deserialize, Debug)]
pub struct DownloadInfo {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Downloads {
    pub client: DownloadInfo,
    pub client_mappings: DownloadInfo,
    pub server: DownloadInfo,
    pub server_mappings: DownloadInfo,
}

#[derive(Deserialize, Debug)]
pub struct VersionInfo {
    pub assets: String,
    pub id: String,
    pub downloads: Downloads,
    pub libraries: Vec<Library>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndex {
    pub id: String,
    #[serde(flatten)]
    pub download_info: DownloadInfo,
    pub total_size: u64,
}

#[derive(Deserialize, Debug)]
pub struct LibraryDownloads {
    pub artifact: LibraryArtifact,
}

#[derive(Deserialize, Debug)]
pub struct Library {
    pub name: String,
    pub downloads: LibraryDownloads,
}

#[derive(Deserialize, Debug)]
pub struct LibraryArtifact {
    pub path: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

pub async fn fetch_manifest() -> reqwest::Result<VersionManifest> {
    let res = reqwest::get("https://piston-meta.mojang.com/mc/game/version_manifest.json").await?;
    res.json::<VersionManifest>().await
}

pub async fn download_version(version: &Version, out_dir: &Path) -> anyhow::Result<()> {
    let version_dir = out_dir.join(&version.id);
    if !version_dir.exists() {
        fs::create_dir_all(&version_dir)?;
    }
    let libs_dir = version_dir.join("libs");
    if !libs_dir.exists() {
        fs::create_dir_all(&libs_dir)?;
    }

    println!("Fetching version info {}...", version.id);
    let version_info = reqwest::get(&version.url).await?.json::<VersionInfo>().await?;

    for lib_info in version_info.libraries {
        println!("Downloading {}", lib_info.name);
        let lib_path = libs_dir.join(lib_info.downloads.artifact.path);
        let size = lib_info.downloads.artifact.size;
        let sha1 = lib_info.downloads.artifact.sha1;
        let url = lib_info.downloads.artifact.url;

        if lib_path.exists() && lib_path.metadata()?.len() == size && sha1_file(&lib_path)? == sha1 {
            println!("Skipping {}", lib_info.name);
            continue;
        }

        lib_path.parent().map(fs::create_dir_all);

        let mut bytes = Cursor::new(reqwest::get(url).await?.bytes().await?);
        io::copy(&mut bytes, &mut File::create(lib_path)?)?;
    }

    Ok(())
}

pub fn sha1_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    use sha1::digest::Digest;
    let mut sha1 = sha1::Sha1::new();
    let mut file = File::open(path)?;
    io::copy(&mut file, &mut sha1)?;
    let mut result = GenericArray::from([0_u8; 20]);
    Digest::finalize_into(sha1, &mut result);

    Ok(hex::encode(result))
}
