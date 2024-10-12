// pub struct GameAssets {
//     version: String
// }

// impl GameAssets {

    
// }

use serde::{Serialize, Deserialize};
use crate::util::{fetch, metadata::API_URL};

pub async fn get_game_assets_version_menifest(server_id: &str, children_server_id: &str, version: &str) -> crate::Result<IAssetsVersionMenifest> {
    let version_menifest = fetch::request_json::<IAssetsVersionMenifest>(format!("{}/launcher/servers/{}/children/{}/versions/{}/metadata", API_URL, server_id, children_server_id, version).as_str()).await?;
    Ok(version_menifest)
}

// #[derive(Debug, Deserialize, Serialize)]
// pub struct IGameAssetsVersionMenifest {
    
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IAssetsVersionMenifest {
    pub version: String,
    pub name: String,
    pub size: i64,
    #[serde(rename = "serverId")]
    pub server_id: String,
    #[serde(rename = "childrenServerId")]
    pub children_server_id: String,
    pub minecraft: IAssetsVersionMenifestMinecraft,
    pub modloader: IAssetsVersionMenifestModloader,
    pub files: Vec<IAssetsVersionMenifestFile>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IAssetsVersionMenifestMinecraft {
    pub version: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IAssetsVersionMenifestModloader {
    #[serde(rename = "type")]
    pub r#type: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IAssetsVersionMenifestFile {
    pub name: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "projectId")]
    pub project_id: Option<u32>,
    #[serde(rename = "fileId")]
    pub file_id: Option<u32>,
    pub platform: Option<String>,
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
    #[serde(rename = "installPath")]
    pub install_path: String,
    pub size: u32,
    pub hash: String,
}
