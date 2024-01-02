use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum ActionRule {
    ALL,
    Whitelist,
    Blacklist
}

#[derive(Debug, Deserialize)]
pub struct ActionPlayer {
    pub name: String,
    pub uuid: String,
}

#[derive(Debug, Deserialize)]
pub struct Action {
    pub rule: ActionRule,
    pub players: Vec<ActionPlayer>
}

#[derive(Debug, Deserialize)]
pub struct Modpack {
    pub name: String,
    pub version: String,
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Children {
    pub name: String,
    #[serde(rename = "minecraftType")]
    pub minecraft_type: String,
    #[serde(rename = "minecraftVersion")]
    pub minecraft_version: String,
    pub action: Action,
    pub modpack: Modpack
}

#[derive(Debug, Deserialize)]
pub struct Servers {
    pub id: String,
    pub name: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub description: String,
    #[serde(rename = "officialWebLinkUrl")]
    pub official_web_link_url: String,
    pub children: Vec<Children>
}

#[derive(Debug, Deserialize)]
pub struct LauncherAssets {
    pub date: String,
    pub servers: Vec<Servers>
}

impl LauncherAssets {
    
}