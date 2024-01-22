use std::path::Path;

use serde::Deserialize;

use crate::minecraft::version::VanillaVersionInfo;
use crate::store::launcher_assets::LauncherAssets;
use crate::util::app_path;
use crate::util::io;

use super::fetch;

pub const S3_MANIFEST_URL: &str = "https://namelessrealms-daedalus.s3.ap-northeast-1.amazonaws.com";
pub const MINECRAFT_VERSION_MANIFEST_URL: &str =
    "https://launchermeta.mojang.com/mc/game/version_manifest.json";

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
    Ok(fetch::request_json::<MinecraftManifest>(MINECRAFT_VERSION_MANIFEST_URL).await?)
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
    let forge_manifest_v0_url = format!("{}/{}", S3_MANIFEST_URL, "forge/v0/manifest.json");
    let forge_manifest_v0 =
        fetch::request_json::<daedalus::modded::Manifest>(&forge_manifest_v0_url).await?;

    Ok(forge_manifest_v0)
}

#[tracing::instrument]
pub async fn get_launcher_assets() -> crate::Result<LauncherAssets> {
    // let home_dir = home::home_dir().unwrap().join("Desktop").join("Projects").join("launchers").join("TS-Launcher").join("allay_core").join("launcher_api.json");
    let home_dir = project_root::get_project_root()?
        .join("allay_core")
        .join("launcher_api.json");
    let ts_launcher_assets = io::read_json_file::<LauncherAssets>(&home_dir).await?;
    Ok(ts_launcher_assets)
}
