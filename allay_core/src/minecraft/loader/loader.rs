use std::{path::{Path, PathBuf}, collections::HashMap};

use serde::Deserialize;

use crate::{loader::fabric::handler::FabricHandler, minecraft::{arguments::Argument, libraries::LibrariesJar, loader::forge::handler::ForgeHandler, version::VanillaVersionInfo}};

use super::forge::handler::ForgeLoader;

#[derive(Debug, Deserialize)]
pub enum LoaderJvmArgumentType {
    Space,
    Equal
}

#[derive(Debug, Deserialize)]
pub struct LoaderJvmArgument {
    pub r#type: LoaderJvmArgumentType,
    pub name: String,
    pub value: String,
    pub keys: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct LoaderArguments {
    pub game: Vec<Argument>,
    pub jvm: Option<Vec<LoaderJvmArgument>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum LoaderType {
    Fabric,
    Forge
}

#[derive(Debug, Deserialize)]
pub struct LoaderVersionInfo {
    pub r#type: LoaderType,
    pub id: String,
    pub loader_version: String,
    #[serde(rename = "mainClass")]
    pub main_class: String,
    pub arguments: LoaderArguments,
    pub libraries: Vec<LibrariesJar>,
    pub forge: Option<ForgeLoader>,
}

pub struct BuildModLoader<'a> {
    loader_version: &'a str,
    minecraft_version: &'a str,
    loader_type: LoaderType,
    version_metadata: &'a VanillaVersionInfo,
}

impl<'a> BuildModLoader<'a> {
    
    #[tracing::instrument]
    pub fn new(minecraft_version: &'a str, loader_type: LoaderType, loader_version: &'a str, version_metadata: &'a VanillaVersionInfo) -> Self {
        BuildModLoader {
            loader_version,
            minecraft_version,
            loader_type,
            version_metadata,
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_loader_version_info(&self) -> crate::Result<LoaderVersionInfo> {

        tracing::info!("Loader type: {:?}", self.loader_type);
        tracing::info!("Loader version: {}", self.loader_version);

        if self.loader_type == LoaderType::Forge {

            let forge_loader_version_info = ForgeHandler::new(self.minecraft_version, self.loader_version, self.version_metadata).get_forge_loader_version_info().await?;

            return Ok(LoaderVersionInfo {
                r#type: self.loader_type.clone(),
                id: self.loader_version.to_owned(),
                loader_version: forge_loader_version_info.loader_version,
                main_class: forge_loader_version_info.main_class,
                arguments: forge_loader_version_info.arguments,
                libraries: forge_loader_version_info.libraries,
                forge: Some(ForgeLoader {
                    client_lzma: forge_loader_version_info.client_lzma,
                    loader_install: forge_loader_version_info.loader_install
                })
            });
        }

        if self.loader_type == LoaderType::Fabric {

            let fabric_loader_version_info = FabricHandler::new(self.minecraft_version, self.loader_version).get_fabric_loader_version_info().await?;

            return Ok(LoaderVersionInfo {
                r#type: self.loader_type.clone(),
                id: self.loader_version.to_owned(),
                loader_version: fabric_loader_version_info.loader_version,
                main_class: fabric_loader_version_info.main_class,
                arguments: fabric_loader_version_info.arguments,
                libraries: fabric_loader_version_info.libraries,
                forge: None
            });
        }

        Err(crate::ErrorKind::LoaderError("Not support loader.".to_owned()).as_error())
    }
}

#[tracing::instrument]
pub fn parse_group_url(url: &str, group: &str) -> crate::Result<String> {
    Ok(format!("{}{}", url, &parse_group_relative(group, ".jar")?.to_string_lossy().to_string()))
}

#[tracing::instrument]
/// Default ext .jar
pub fn parse_group_relative_path(group: &str) -> crate::Result<PathBuf> {
    Ok(parse_group_relative(group, ".jar")?)
}

#[tracing::instrument]
pub fn parse_group_relative_path_ext(group: &str, ext: &str) -> crate::Result<PathBuf> {
    Ok(parse_group_relative(group, ext)?)
}

#[tracing::instrument]
fn parse_group_relative(group: &str, ext: &str) -> crate::Result<PathBuf> {

    let ext_replace = ext.replace(".", "");
    let ext = ext_replace.as_str();

    // *  Input 1: net.minecraftforge:forge:1.16.5-36.2.39
    // * Output 1: net/minecraftforge/forge/1.16.5-36.2.39/forge-1.16.5-36.2.39.jar

    // ? @txt <- 副檔名 ext
    // *  Input 2: de.oceanlabs.mcp:mcp_config:1.16.5-20210115.111550:mappings@txt
    // * Output 2: de/oceanlabs/mcp/mcp_config/1.16.5-20210115.111550/mcp_config-1.16.5-20210115.111550-mappings.txt

    // *  Input 3: net.minecraft:client:1.16.5-20210115.111550:slim
    // * Output 3: net/minecraft/client/1.16.5-20210115.111550/client-1.16.5-20210115.111550-slim.jar

    // *  Input 4: de.oceanlabs.mcp:mcp_config:1.16.5-20210115.111550@zip
    // * Output 4: de/oceanlabs/mcp/mcp_config/1.16.5-20210115.111550/mcp_config-1.16.5-20210115.111550.zip

    // ? Result 1: [net.minecraftforge],[forge],[1.16.5-36.2.39] len 3
    // ? Result 2: [de.oceanlabs.mcp],[mcp_config],[1.16.5-20210115.111550],[mappings@txt] len 4
    // ? Result 3: [net.minecraft],[client],[1.16.5-20210115.111550],[slim] len 4
    // ? Result 4: [de.oceanlabs.mcp],[mcp_config],[1.16.5-20210115.111550@zip] len 3
    let split = group.split(":").collect::<Vec<&str>>();

    // ? Result 1: net/minecraftforge/forge/1.16.5-36.2.39
    // ? Result 2: de/oceanlabs/mcp/mcp_config/1.16.5-20210115.111550
    // ? Result 3: net/minecraft/client/1.16.5-20210115.111550
    // ? Result 4: de/oceanlabs/mcp/mcp_config/1.16.5-20210115.111550
    let file_dir_relative_path = Path::new(&split[0].split(".").collect::<Vec<&str>>().join("/")).join(split[1]).join(split[2].split("@").collect::<Vec<&str>>()[0]);

    // ? Result: 1,4
    if split.len() <= 3 {

        let mouse_split = split.last().unwrap().split("@").collect::<Vec<&str>>();

        let mut ext = ext;

        if mouse_split.len() == 2 {
            ext = *mouse_split.last().unwrap();
        }

        let file_name = format!("{}-{}.{}", split[split.len() - 2], mouse_split.first().unwrap(), ext);

        return Ok(file_dir_relative_path.join(file_name));
    }

    // ? Result: 2,3
    if split.len() == 4 {
        
        // ? Result 2: [mappings] [txt]
        let mouse_split = split.last().unwrap().split("@").collect::<Vec<&str>>();

        let mut ext = ext;

        if mouse_split.len() == 2 {
            ext = *mouse_split.last().unwrap();
        }

        let file_name = format!("{}-{}-{}.{}", split[split.len() - 3], split[split.len() - 2], mouse_split.first().unwrap(), ext);

        return Ok(file_dir_relative_path.join(file_name));
    }

    Err(crate::ErrorKind::LoaderError("Parse group relative error".to_owned()).as_error())
}

#[cfg(test)]
mod tests {

    use std::path::Path;
    use crate::minecraft::loader::loader::{parse_group_url, parse_group_relative_path};

    #[test]
    fn it_parse_group_relative() -> crate::Result<()> {
        
        // * Test sample 1
        let binding = parse_group_relative_path("net.minecraftforge:forge:1.16.5-36.2.39")?;
        let relative_path = binding.to_string_lossy();
        let output_relative_path = Path::new("net/minecraftforge/forge/1.16.5-36.2.39/forge-1.16.5-36.2.39.jar").to_string_lossy();
        assert_eq!(relative_path, output_relative_path);

        // * Test sample 2
        let binding = parse_group_relative_path("de.oceanlabs.mcp:mcp_config:1.16.5-20210115.111550:mappings@txt")?;
        let relative_path = binding.to_string_lossy();
        let output_relative_path = Path::new("de/oceanlabs/mcp/mcp_config/1.16.5-20210115.111550/mcp_config-1.16.5-20210115.111550-mappings.txt").to_string_lossy();
        assert_eq!(relative_path, output_relative_path);

        // * Test sample 3
        let binding = parse_group_relative_path("net.minecraft:client:1.16.5-20210115.111550:slim")?;
        let relative_path = binding.to_string_lossy();
        let output_relative_path = Path::new("net/minecraft/client/1.16.5-20210115.111550/client-1.16.5-20210115.111550-slim.jar").to_string_lossy();
        assert_eq!(relative_path, output_relative_path);

        // * Test sample 4
        let binding = parse_group_relative_path("de.oceanlabs.mcp:mcp_config:1.16.5-20210115.111550@zip")?;
        let relative_path = binding.to_string_lossy();
        let output_relative_path = Path::new("de/oceanlabs/mcp/mcp_config/1.16.5-20210115.111550/mcp_config-1.16.5-20210115.111550.zip").to_string_lossy();
        assert_eq!(relative_path, output_relative_path);

        Ok(())
    }

    #[test]
    fn it_parse_group_url() -> crate::Result<()> {

        let url = parse_group_url("https://namelessrealms-daedalus.s3.ap-northeast-1.amazonaws.com/maven/", "net.minecraftforge:forge:1.16.5-36.2.39")?;
        assert_eq!(url, "https://namelessrealms-daedalus.s3.ap-northeast-1.amazonaws.com/maven/net/minecraftforge/forge/1.16.5-36.2.39/forge-1.16.5-36.2.39.jar");

        Ok(())
    }
}