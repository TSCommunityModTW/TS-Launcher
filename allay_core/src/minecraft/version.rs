// #![allow(dead_code)]
// #![allow(unused_variables)]

use std::{fmt::format, path::{Path, PathBuf}};

use serde::{Deserialize, Serialize};

use crate::util::app_path;

use super::{libraries, arguments::MinecraftArguments, assets::{self, AssetObjects}};

#[derive(Debug, Deserialize, Serialize)]
pub struct Arguments {
    pub game: Vec<serde_json::Value>,
    pub jvm: Vec<serde_json::Value>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: u32,
    #[serde(rename = "totalSize")]
    pub total_size: u32,
    pub url: String
}

#[derive(Debug, Deserialize)]
pub struct ClientJar {
    pub name: String,
    pub relative_path: PathBuf,
    pub path: PathBuf,
    pub sha1: String,
    pub size: u32,
    pub download_url: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DownloadsClient {
    pub sha1: String,
    pub size: u32,
    #[serde(rename = "url")]
    pub download_url: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Downloads {
    client: DownloadsClient,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_mappings: Option<DownloadsClient>,
    server: DownloadsClient,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_mappings: Option<DownloadsClient>
}

#[derive(Debug, Deserialize, Serialize)]
struct JavaVersion {
    component: String,
    #[serde(rename = "majorVersion")]
    major_version: u32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LibrariesFile {
    pub path: String,
    pub sha1: String,
    pub size: u32,
    pub url: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LibrariesClassifiers {
    #[serde(rename = "natives-linux", skip_serializing_if = "Option::is_none")]
    pub natives_linux: Option<LibrariesFile>,
    #[serde(rename = "natives-macos", skip_serializing_if = "Option::is_none")]
    pub natives_macos: Option<LibrariesFile>,
    #[serde(rename = "natives-osx", skip_serializing_if = "Option::is_none")]
    pub natives_osx: Option<LibrariesFile>,
    #[serde(rename = "natives-windows", skip_serializing_if = "Option::is_none")]
    pub natives_windows: Option<LibrariesFile>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LibrariesDownloads {
    pub artifact: Option<LibrariesFile>,
    pub classifiers: Option<LibrariesClassifiers>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LibrariesRulesOS {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LibrariesRules {
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<LibrariesRulesOS>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LibrariesNatives {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osx: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Libraries {
    pub downloads: LibrariesDownloads,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natives: Option<LibrariesNatives>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<LibrariesRules>>
}

#[derive(Debug, Deserialize, Serialize)]
struct LoggingClientFile {
    id: String,
    sha1: String,
    size: u32,
    url: String
}

#[derive(Debug, Deserialize, Serialize)]
struct LoggingClient {
    argument: String,
    file: LoggingClientFile,
    r#type: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Logging {
    client: LoggingClient
}

#[derive(Debug)]
pub enum VersionTypes {
    Release,
    Snapshot,
    OldAlpha,
    Null
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VanillaVersionInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    arguments: Option<Arguments>,
    #[serde(rename = "assetIndex")]
    asset_index: AssetIndex,
    assets: String,
    #[serde(rename = "complianceLevel", skip_serializing_if = "Option::is_none")]
    compliance_level: Option<u32>,
    downloads: Downloads,
    id: String,
    #[serde(rename = "javaVersion", skip_serializing_if = "Option::is_none")]
    java_version: Option<JavaVersion>,
    libraries: Vec<Libraries>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<Logging>,
    #[serde(rename = "mainClass")]
    main_class: String,
    #[serde(rename = "minecraftArguments", skip_serializing_if = "Option::is_none")]
    minecraft_arguments: Option<String>,
    #[serde(rename = "minimumLauncherVersion")]
    minimum_launcher_version: i32,
    #[serde(rename = "releaseTime")]
    release_time: String,
    time: String,
    r#type: String
}

impl VanillaVersionInfo {

    pub fn get_java_parameters(&self) -> MinecraftArguments {
        
        let higher_version = &self.arguments;
        let lower_version = &self.minecraft_arguments;
        let version = &self.id;

        MinecraftArguments { higher_version, lower_version, version }
    }

    pub fn get_libraries(&self) -> Vec<libraries::LibrariesJar> {
        libraries::is_libraries(&self.libraries)
    }

    pub async fn get_asset_objects(&self) -> crate::Result<Vec<AssetObjects>> {
        let asset_objects = assets::get_asset_objects(&self.get_asset_index(), &self.get_assets_index_id()).await?;
        Ok(asset_objects)
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_assets_index_id(&self) -> &str {
        &self.assets
    }

    pub fn get_asset_index(&self) -> &AssetIndex {
        &self.asset_index
    }

    pub fn get_client_jar(&self) -> ClientJar {
        let minecraft_version = self.get_id();
        let client_jar_file_name = format!("{}.jar", minecraft_version);
        let relative_path = Path::new(minecraft_version).join(&client_jar_file_name);
        ClientJar {
            name: client_jar_file_name.to_string(),
            relative_path: relative_path.to_path_buf(),
            path: app_path::combine_common_paths_absolute(Path::new("versions"), &relative_path),
            sha1: self.downloads.client.sha1.to_string(),
            size: self.downloads.client.size,
            download_url: self.downloads.client.download_url.to_string()
            // download_url: format!("{}{}", self.downloads.client.download_url.to_string(), "x") // TODO: ERROR TEST
        }
    }

    pub fn get_java_vm_version(&self) -> &u32 {
        // TODO
        &self.java_version.as_ref().unwrap().major_version
    }

    pub fn get_main_class_name(&self) -> &str {
        &self.main_class
    }

    pub fn get_release_time(&self) -> &str {
        &self.release_time
    }

    pub fn get_type(&self) -> &VersionTypes {
        match self.r#type.as_str() {
            "release" => &VersionTypes::Release,
            "snapshot" => &VersionTypes::Snapshot,
            "old_alpha" => &VersionTypes::OldAlpha,
            _ => &VersionTypes::Null
        }
    }
}