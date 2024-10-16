use std::{collections::HashMap, path::Path};

use regex::Regex;
use serde::Deserialize;

use crate::{FORGE_MANIFEST_V1_QUERY, FORGE_MANIFEST_V2_QUERY, FORGE_MANIFEST_V2_QUERY_P1, FORGE_MANIFEST_V3_QUERY_P1, minecraft::{libraries::{LibrariesJar, LibrariesJarType}, loader::loader::{LoaderArguments, parse_group_relative_path, parse_group_url, LoaderJvmArgument, LoaderJvmArgumentType}, version::VanillaVersionInfo, arguments::{Argument, self}}, util::{app_path, metadata, fetch}};

#[derive(Debug, Deserialize)]
pub struct LoaderInstall {
    pub data: HashMap<String, daedalus::modded::SidedDataEntry>,
    pub processors: Vec<daedalus::modded::Processor>
}

#[derive(Debug, Deserialize)]
pub struct ForgeLibrariesJarV2 {
    libraries: Vec<LibrariesJar>,
    client_lzma: Option<LibrariesJar>
}

#[derive(Debug, Deserialize)]
pub struct ForgeLoader {
    pub client_lzma: Option<LibrariesJar>,
    pub loader_install: Option<LoaderInstall>
}

#[derive(Debug, Deserialize)]
pub struct ForgeLoaderVersionInfo {
    pub loader_version: String,
    #[serde(rename = "mainClass")]
    pub main_class: String,
    pub arguments: LoaderArguments,
    pub libraries: Vec<LibrariesJar>,
    pub client_lzma: Option<LibrariesJar>,
    pub loader_install: Option<LoaderInstall>
}

pub struct ForgeHandler<'a> {
        loader_version: &'a str,
        minecraft_version: &'a str,
        version_metadata: &'a VanillaVersionInfo,
}

impl<'a> ForgeHandler<'a> {

    #[tracing::instrument]
    pub fn new(minecraft_version: &'a str, loader_version: &'a str, version_metadata: &'a VanillaVersionInfo) -> Self {
        ForgeHandler {
            loader_version,
            minecraft_version,
            version_metadata,
        }
    }

    pub async fn get_forge_loader_version_info(&self) -> crate::Result<ForgeLoaderVersionInfo> {

        let parse_forge_loader_version = parse_forge_loader_version(self.loader_version)?;

        tracing::info!("Forge parse id: {}", parse_forge_loader_version);
        
        let forge_loader_manifest = self.get_forge_version_manifest().await?;

        // let id = format!("{}-forge-{}", self.minecraft_version, self.loader_version);

        let loader_version = format!("{}-{}", "forge", self.loader_version);

        if FORGE_MANIFEST_V1_QUERY.matches(&parse_forge_loader_version) {

            let forge_game_arguments = self.build_forge_game_arguments(&forge_loader_manifest)?;
            let forge_libraries = self.build_forge_libraries_v1(&forge_loader_manifest.libraries)?;

            return Ok(ForgeLoaderVersionInfo {
                loader_version,
                main_class: forge_loader_manifest.main_class.unwrap(),
                arguments: LoaderArguments {
                    game: forge_game_arguments,
                    jvm: None
                },
                libraries: forge_libraries,
                client_lzma: None,
                loader_install: None
            });

        } else if FORGE_MANIFEST_V2_QUERY.matches(&parse_forge_loader_version) || FORGE_MANIFEST_V2_QUERY_P1.matches(&parse_forge_loader_version) {
            
            let forge_game_arguments = self.build_forge_game_arguments(&forge_loader_manifest)?;
            let forge_libraries = self.build_forge_libraries_v2(&forge_loader_manifest.libraries)?;

            let forge_install_data = forge_loader_manifest.data.ok_or_else(|| {
                crate::ErrorKind::LoaderError("Get forge install data error".to_owned())
            })?;

            let forge_install_processors = forge_loader_manifest.processors.ok_or_else(|| {
                crate::ErrorKind::LoaderError("Get forge install processors error".to_owned())
            })?;

            if FORGE_MANIFEST_V2_QUERY.matches(&parse_forge_loader_version) {
                return Ok(ForgeLoaderVersionInfo {
                    loader_version,
                    main_class: forge_loader_manifest.main_class.unwrap(),
                    arguments: LoaderArguments {
                        game: forge_game_arguments,
                        jvm: None
                    },
                    libraries: forge_libraries.libraries,
                    client_lzma: forge_libraries.client_lzma,
                    loader_install: None
                });
            } else {
                return Ok(ForgeLoaderVersionInfo {
                    loader_version,
                    main_class: forge_loader_manifest.main_class.unwrap(),
                    arguments: LoaderArguments {
                        game: forge_game_arguments,
                        jvm: None
                    },
                    libraries: forge_libraries.libraries,
                    client_lzma: forge_libraries.client_lzma,
                    loader_install: Some(LoaderInstall {
                        data: forge_install_data,
                        processors: forge_install_processors
                    })
                });
            }

        } else if FORGE_MANIFEST_V3_QUERY_P1.matches(&parse_forge_loader_version) {

            let forge_game_arguments = self.build_forge_game_arguments(&forge_loader_manifest)?;
            let forge_jvm_arguments = self.build_forge_jvm_arguments(&forge_loader_manifest)?;
            let forge_libraries = self.build_forge_libraries_v2(&forge_loader_manifest.libraries)?;

            let forge_install_data = forge_loader_manifest.data.ok_or_else(|| {
                crate::ErrorKind::LoaderError("Get forge install data error".to_owned())
            })?;

            let forge_install_processors = forge_loader_manifest.processors.ok_or_else(|| {
                crate::ErrorKind::LoaderError("Get forge install processors error".to_owned())
            })?;

            return Ok(ForgeLoaderVersionInfo {
                loader_version,
                main_class: forge_loader_manifest.main_class.unwrap(),
                arguments: LoaderArguments {
                    game: forge_game_arguments,
                    jvm: Some(forge_jvm_arguments)
                },
                libraries: forge_libraries.libraries,
                client_lzma: forge_libraries.client_lzma,
                loader_install: Some(LoaderInstall {
                    data: forge_install_data,
                    processors: forge_install_processors
                })
            });
        }

        Err(crate::ErrorKind::LoaderError("Forge not compliant version.".to_owned()).as_error())
    }

     /// 處理 Forge 23.5.2851 >= ? libraries
     fn build_forge_libraries_v2(&self, forge_libraries: &Vec<daedalus::minecraft::Library>) -> crate::Result<ForgeLibrariesJarV2> {

        // println!("{:#?}", forge_libraries);

        let client_lzma_regex = Regex::new("client@lzma").unwrap();
        let mut client_lzma = None;

        let mut forge_libraries_result: Vec<LibrariesJar> = Vec::new();

        for forge_lib in forge_libraries.iter() {

            // TODO: json key: downloads, If None skip
            if forge_lib.downloads.is_none() {

                // * If client@lzma
                if client_lzma_regex.is_match(&forge_lib.name) {

                    let relative_path = parse_group_relative_path(&forge_lib.name)?;

                    let url = forge_lib.url.as_ref().ok_or_else(|| {
                        crate::ErrorKind::LoaderError("Get client@lzma url error".to_owned())
                    })?;

                    client_lzma = Some(LibrariesJar {
                        r#type: LibrariesJarType::ModLoaderLzma,
                        name: forge_lib.name.to_owned(),
                        relative_path: relative_path.to_path_buf(),
                        path: app_path::combine_common_paths_absolute(Path::new("libraries"), &relative_path),
                        sha1: "".to_owned(),
                        size: 0,
                        download_url: parse_group_url(&url, &forge_lib.name)?,
                        relative_url: None,
                        manifest_url: None,
                        include_in_classpath: forge_lib.include_in_classpath,
                    });
                }

                continue;
            }

            let download = &forge_lib.downloads.clone().ok_or_else(|| {
                crate::ErrorKind::LoaderError("Get Forge 23.5.2851 >= ? downloads error".to_owned())
            })?;

            let artifact = download.artifact.clone().ok_or_else(|| {
                crate::ErrorKind::LoaderError("Get Forge 23.5.2851 >= ? artifact error".to_owned())
            })?;

            let path = artifact.path.unwrap();
            let file_path = Path::new(&path);
            
            forge_libraries_result.push(LibrariesJar {
                r#type: LibrariesJarType::ModLoader,
                name: forge_lib.name.to_owned(),
                relative_path: file_path.to_path_buf(),
                path: app_path::combine_common_paths_absolute(Path::new("libraries"), &file_path),
                sha1: artifact.sha1,
                size: artifact.size,
                download_url: artifact.url,
                relative_url: None,
                manifest_url: None,
                include_in_classpath: forge_lib.include_in_classpath
            });
        }

        Ok(ForgeLibrariesJarV2 {
            libraries: forge_libraries_result,
            client_lzma
        })
    }

    /// 處理 Forge 8.0.684 >= 23.5.2851 libraries
    fn build_forge_libraries_v1(&self, forge_libraries: &Vec<daedalus::minecraft::Library>) -> crate::Result<Vec<LibrariesJar>> {

        let mut forge_libraries_result: Vec<LibrariesJar> = Vec::new();

        for forge_lib in forge_libraries.iter() {

            // TODO:
            // if forge_lib.url.is_none() && forge_lib.name.split(":").collect::<Vec<&str>>()[1] != "launchwrapper" {
            //     continue;
            // }

            let url = forge_lib.url.clone().unwrap_or_else(|| {
                "https://namelessrealms-daedalus.s3.ap-northeast-1.amazonaws.com/maven/".to_owned()
            });

            let file_url = parse_group_url(&url, &forge_lib.name)?;
            let relative_file_path = &parse_group_relative_path(&forge_lib.name)?;

            forge_libraries_result.push(LibrariesJar {
                r#type: LibrariesJarType::ModLoader,
                name: forge_lib.name.to_owned(),
                relative_path: relative_file_path.to_path_buf(),
                path: app_path::combine_common_paths_absolute(Path::new("libraries"), relative_file_path),
                sha1: "".to_owned(),
                size: 0,
                download_url: file_url,
                relative_url: Some(relative_file_path.to_string_lossy().to_string()),
                manifest_url: Some(["https://maven.minecraftforge.net/".to_owned(), "https://libraries.minecraft.net/".to_owned()].to_vec()),
                include_in_classpath: forge_lib.include_in_classpath
            });
        }

        Ok(forge_libraries_result)
    }

    // * 處理 Forge Jvm 36.1.66 >= ?
    #[tracing::instrument(skip(self, forge_arguments))]
    fn build_forge_jvm_arguments(&self, forge_arguments: &daedalus::modded::PartialVersionInfo) -> crate::Result<Vec<LoaderJvmArgument>> {
        
        let mut forge_jvm_arguments: Vec<LoaderJvmArgument> = Vec::new();

        let jvm_arguments = {
            let arguments = forge_arguments.arguments.as_ref().ok_or_else(|| {
                crate::ErrorKind::LoaderError("TODO".to_owned())
            })?;
            let jvm_arguments = arguments.get(&daedalus::minecraft::ArgumentType::Jvm).ok_or_else(|| {
                crate::ErrorKind::LoaderError("TODO".to_owned())
            })?;
            jvm_arguments
        };

        let mut name: Option<&str> = None;
        let regex_1 = Regex::new(r"\$\{[^}]*\}").unwrap();
        let regex_2 = Regex::new(r"^-[^-].*$").unwrap();
        let regex_3 = Regex::new(r"^-[^=]*=[^=]*$").unwrap();
        let regex_4 = Regex::new(r"^--").unwrap();

        for jvm_argument in jvm_arguments.iter() {
            match jvm_argument {
                daedalus::minecraft::Argument::Normal(jvm_value) => {

                    if name.is_some() {

                        {
                            let name = name.ok_or_else(|| {
                                crate::ErrorKind::LoaderError("Get name error.".to_owned())
                            })?;
    
                            if !regex_4.is_match(name) {
    
                                let mut variables: HashMap<String, String> = HashMap::new();
    
                                for capture in regex_1.captures_iter(jvm_value) {
                                    if let Some(variable) = capture.get(0) {
                                        variables.insert(variable.as_str().to_owned(), "".to_string());
                                    }
                                }
    
                                forge_jvm_arguments.push(LoaderJvmArgument {
                                    r#type: LoaderJvmArgumentType::Space,
                                    name: name.to_owned(),
                                    value: jvm_value.to_owned(),
                                    keys: Some(variables),
                                })
                            } else {
                                
                                forge_jvm_arguments.push(LoaderJvmArgument {
                                    r#type: LoaderJvmArgumentType::Space,
                                    name: name.to_owned(),
                                    value: jvm_value.to_owned(),
                                    keys: None,
                                })

                            }
                        }

                        name = None;
                    }

                    if regex_2.is_match(&jvm_value) {

                        if regex_3.is_match(&jvm_value) {
                            
                            let split = jvm_value.split("=").collect::<Vec<&str>>();

                            if regex_1.is_match(split[1]) {

                                let mut variables: HashMap<String, String> = HashMap::new();

                                for capture in regex_1.captures_iter(split[1]) {
                                    if let Some(variable) = capture.get(0) {
                                        variables.insert(variable.as_str().to_owned(), "".to_string());
                                    }
                                }

                                forge_jvm_arguments.push(LoaderJvmArgument {
                                    r#type: LoaderJvmArgumentType::Equal,
                                    name: split[0].to_owned(),
                                    value: split[1].to_owned(),
                                    keys: Some(variables),
                                })

                            } else {
                                forge_jvm_arguments.push(LoaderJvmArgument {
                                    r#type: LoaderJvmArgumentType::Equal,
                                    name: split[0].to_owned(),
                                    value: split[1].to_owned(),
                                    keys: None,
                                })
                            }

                        } else {
                            
                            name = Some(jvm_value);

                        }

                    }

                    if regex_4.is_match(&jvm_value) {
                        name = Some(jvm_value);
                    }

                },
                _ => tracing::warn!("TODO Loader rules")
            }
        }

        // println!("{:#?}", forge_jvm_arguments);

        Ok(forge_jvm_arguments)
    }

    // 處理 Forge 遊戲參數
    #[tracing::instrument(skip(self, forge_arguments))]
    fn build_forge_game_arguments(&self, forge_arguments: &daedalus::modded::PartialVersionInfo) -> crate::Result<Vec<Argument>> {
        
        let modloader_version = parse_forge_loader_version(self.loader_version)?;
        let mut forge_game_arguments: Vec<Argument> = Vec::new();

        if semver::VersionReq::parse(">=23.5.2852, <=23.5.2860").unwrap().matches(&modloader_version) {

            // * 處理 Forge 23.5.2852 >= 23.5.2860
            let minecraft_arguments = forge_arguments.minecraft_arguments.clone().ok_or_else(|| {
                crate::ErrorKind::LoaderError("處理 Forge 23.5.2852 >= 23.5.2860, Get minecraftArguments error".to_owned())
            })?;
            forge_game_arguments = arguments::extract_parameters_from_arguments(&minecraft_arguments);

        } else {

            // * 處理 Forge 8.0.684 >= 23.5.2851
            let forge_arguments = {
                let arguments = forge_arguments.arguments.as_ref().ok_or_else(|| {
                    crate::ErrorKind::LoaderError("TODO".to_owned())
                })?;
                let game_arguments = arguments.get(&daedalus::minecraft::ArgumentType::Game).ok_or_else(|| {
                    crate::ErrorKind::LoaderError("TODO".to_owned())
                })?;
                game_arguments
            };
    
            forge_game_arguments = {
    
                let mut forge_game_arguments: Vec<Argument> = Vec::new();
    
                let mut name = "".to_owned();
                let regex_1 = Regex::new(r"\$\{[^}]*\}").unwrap();
                let regex_2 = Regex::new(r"^--").unwrap();
    
                for forge_argument in forge_arguments.iter() {
                    match forge_argument {
                        daedalus::minecraft::Argument::Normal(value) => {
    
                            if regex_1.is_match(value) {
                                forge_game_arguments.push(Argument {
                                    name: name.clone(),
                                    key: value.to_string(),
                                    value: "".to_owned()
                                });
                            } else if regex_2.is_match(value) {
                                name = value.to_string();
                            } else {
                                forge_game_arguments.push(Argument {
                                    name: name.clone(),
                                    key: "".to_owned(),
                                    value: value.to_string()
                                });
                            }
    
                        },
                        _ => tracing::warn!("TODO Loader rules")
                    }
                }
                forge_game_arguments
            };
        }

        tracing::debug!("Forge game arguments: {:?}", forge_game_arguments);

        let vanilla_game_arguments = self.version_metadata.get_java_parameters().get_game().arguments;
        let mut forge_game_arguments: Vec<crate::minecraft::arguments::Argument> = forge_game_arguments.iter().filter_map(|v| {
            if vanilla_game_arguments.iter().find(|v2| v2.name == v.name).is_some() {
                None
            } else {
                Some(v)
            }
        }).cloned().collect();

        tracing::debug!("Forge game arguments filter: {:?}", forge_game_arguments);

        let forge_game_arguments: Vec<Argument> = forge_game_arguments
            .iter_mut()
            .map(|forge_argument| {
                let val = match forge_argument.key.as_str() {
                    // TODO
                    _ => forge_argument.value.clone(),
                };
                forge_argument.value = val;
                forge_argument.clone()
            }).collect();

        tracing::debug!("Forge game arguments add value: {:?}", forge_game_arguments);

        Ok(forge_game_arguments)
    }

    // 提取獲取 Forge 版本信息
    #[tracing::instrument(skip(self))]
    async fn get_forge_version_manifest(&self) -> crate::Result<daedalus::modded::PartialVersionInfo> {

        let forge_manifest = metadata::get_forge_versions_manifest().await?;

        if let Some(modloaders_manifest) = forge_manifest.game_versions.iter().find(|v| v.id == self.minecraft_version) {

            // let id = format!("{}-{}", self.minecraft_version, self.loader_version);
            let id = self.loader_version;
            let modloaders = &modloaders_manifest.loaders;

            let forge_loader_version = modloaders.iter().find(|v| v.id == id)
                .ok_or_else(|| crate::ErrorKind::LoaderError(String::from("獲取模組載入器資料失敗，無法取得 Loader version")))?;

            Ok(fetch::request_json::<daedalus::modded::PartialVersionInfo>(&forge_loader_version.url).await?)

        } else {
            Err(crate::ErrorKind::LoaderError("獲取模組載入器資料失敗，無法取得 Minecraft version loaders".to_owned()).as_error())
        }
    }
}

// 提取解析 Mod Loader 版本
#[tracing::instrument]
pub fn parse_forge_loader_version(loader_version: &str) -> crate::Result<semver::Version> {

    let mut split = loader_version.split('.').collect::<Vec<&str>>();

    let modloader_version = if split.len() >= 4 {

        // ! Flx 10.13.4.1614-1.7.10 -> 13.4.1614-1 -> 13.4.1614
        let new_text = &split[3].replace("-1", "");
        split[3] = new_text;

        if split[0].parse::<i32>().unwrap_or(0) < 6 {
            format!("{}.{}.{}", split[0], split[1], split[3])
        } else {
            format!("{}.{}.{}", split[1], split[2], split[3])
        }
    } else {
        loader_version.to_owned()
    };

    Ok(semver::Version::parse(&modloader_version)?)
}