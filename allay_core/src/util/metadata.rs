use std::path::Path;

use serde::Deserialize;

use crate::minecraft::version::VanillaVersionInfo;
use crate::store::launcher_assets::LauncherAssets;
use crate::util::app_path;
use crate::util::io;

use super::fetch;

pub const S3_URL: &str = "https://s3api.tshosts.com/minecraft-metadata";
#[cfg(not(debug_assertions))]
pub const API_URL: &str = "https://api.tshosts.com";
#[cfg(debug_assertions)]
pub const API_URL: &str = " http://localhost:8030";

#[derive(Debug, Deserialize)]
pub struct MinecraftManifestLatest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Deserialize)]
pub struct MinecraftManifestVersion {
    pub id: String,
    pub r#type: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
}

#[derive(Debug, Deserialize)]
pub struct MinecraftManifest {
    pub latest: MinecraftManifestLatest,
    pub versions: Vec<MinecraftManifestVersion>,
}

#[tracing::instrument]
async fn get_versions_manifest() -> crate::Result<MinecraftManifest> {
    Ok(fetch::request_json::<MinecraftManifest>("https://launchermeta.mojang.com/mc/game/version_manifest.json").await?)
}

#[tracing::instrument]
pub async fn get_vanilla_version_info(version: &str) -> crate::Result<VanillaVersionInfo> {
    if let Some(versions) = get_versions_manifest()
        .await?
        .versions
        .iter()
        .find(|v| v.id == version)
    {
        let version_url = &versions.url;
        let metadata = fetch::request_json::<VanillaVersionInfo>(&version_url).await?;

        // TODO: write file
        let metadata_path = Path::new(&app_path::get_common_dir_path())
            .join("versions")
            .join(&metadata.get_id())
            .join(format!("{}.json", &metadata.get_id()));
        io::write_struct_file(&metadata_path, &metadata).await?;

        Ok(metadata)
    } else {
        panic!("未找到 Minecraft 版本數據。");
    }
}

#[tracing::instrument]
pub async fn get_forge_versions_manifest() -> crate::Result<daedalus::modded::Manifest> {
    let forge_manifest_v0_url = format!("{}/{}", S3_URL, "forge/v0/manifest.json");
    let forge_manifest_v0 =
        fetch::request_json::<daedalus::modded::Manifest>(&forge_manifest_v0_url).await?;
    Ok(forge_manifest_v0)
}

#[tracing::instrument]
pub async fn get_fabric_versions_manifest() -> crate::Result<daedalus::modded::Manifest> {
    let fabric_manifest_v0_url = format!("{}/{}", S3_URL, "fabric/v0/manifest.json");
    let fabric_manifest_v0 = fetch::request_json::<daedalus::modded::Manifest>(&fabric_manifest_v0_url).await?;
    Ok(fabric_manifest_v0)
}

#[tracing::instrument]
pub async fn get_launcher_assets() -> crate::Result<LauncherAssets> {
    Ok(fetch::request_json::<LauncherAssets>(format!("{}/{}", API_URL, "launcher/servers/metadata").as_str()).await?)
}