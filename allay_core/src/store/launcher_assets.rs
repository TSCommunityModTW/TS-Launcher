use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;
use ts_rs::TS;

use crate::util::metadata;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(rename = "ILASCModpackActionActionRule")]
#[ts(export, export_to = "../ts_gui/src/interfaces/ILASCModpackActionActionRule.ts")]
pub enum ActionRule {
    ALL,
    Whitelist,
    Blacklist
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(rename = "ILASCModpackActionActionPlayer")]
#[ts(export, export_to = "../ts_gui/src/interfaces/ILASCModpackActionActionPlayer.ts")]
pub struct ActionPlayer {
    pub name: String,
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(rename = "ILASChildrenModpackAction")]
#[ts(export, export_to = "../ts_gui/src/interfaces/ILASChildrenModpackAction.ts")]
pub struct Action {
    pub rule: ActionRule,
    pub players: Vec<ActionPlayer>
}

// #[derive(Debug, Serialize, Deserialize, Clone, TS)]
// #[ts(rename = "ILAServerChildrenModpack")]
// #[ts(export, export_to = "../ts_gui/src/interfaces/ILAServerChildrenModpack.ts")]
// pub struct Modpack {
//     pub name: String,
//     pub version: String,
//     #[serde(rename = "downloadUrl")]
//     pub download_url: String,
// }

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(rename = "ILauncherAssetsServerChildrenAnnounment")]
#[ts(export, export_to = "../ts_gui/src/interfaces/ILauncherAssetsServerChildrenAnnounment.ts")]
pub struct ServerAnnounment {
    pub title: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(rename = "ILauncherAssetsServerChildrenAssets")]
#[ts(export, export_to = "../ts_gui/src/interfaces/ILauncherAssetsServerChildrenAssets.ts")]
pub struct Assets {
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(rename = "ILauncherAssetsServerChildren")]
#[ts(export, export_to = "../ts_gui/src/interfaces/ILauncherAssetsServerChildren.ts")]
pub struct ServerChildren {
    pub id: String,
    pub ip: String,
    pub name: String,
    // #[serde(rename = "imageUrl")]
    pub image_url: String,
    // #[serde(rename = "minecraftType")]
    pub minecraft_type: String,
    // #[serde(rename = "minecraftVersion")]
    pub minecraft_version: String,
    pub action: Action,
    // pub modpack: Modpack,
    pub assets: Assets,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(rename = "ILauncherAssetsServer")]
#[ts(export, export_to = "../ts_gui/src/interfaces/ILauncherAssetsServer.ts")]
pub struct Server {
    pub id: String,
    pub name: String,
    // #[serde(rename = "image_url")]
    pub image_url: String,
    pub description: String,
    // #[serde(rename = "officialWebLinkUrl")]
    pub official_web_link_url: String,
    pub children: Vec<ServerChildren>,
    pub announment: Vec<ServerAnnounment>
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(rename = "ILauncherAssets")]
#[ts(export, export_to = "../ts_gui/src/interfaces/ILauncherAssets.ts")]
pub struct LauncherAssets {
    pub date: String,
    pub servers: Vec<Server>
}

static LAUNCHER_ASSETS: OnceCell<LauncherAssets> = OnceCell::const_new();

impl LauncherAssets {
    
    pub async fn initialize() -> crate::Result<()> {
        let _ = LAUNCHER_ASSETS.set( metadata::get_launcher_assets().await?);
        Ok(())
    }

    pub fn get() -> crate::Result<LauncherAssets> {
        if let Some(launcher_assets) = LAUNCHER_ASSETS.get() {
            return Ok(launcher_assets.clone());
        } else {
            Err(crate::ErrorKind::LauncherAssetsError("取得啟動器資源失敗，請先初始化".to_owned()).as_error())
        }
    }

    pub fn get_servers() -> crate::Result<Vec<Server>> {
        Ok(Self::get()?.servers)
    }
}