use serde::{Deserialize, Serialize};

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