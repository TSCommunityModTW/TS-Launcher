use std::{collections::HashMap, path::Path};

use regex::Regex;

use crate::{arguments::Argument, libraries::{LibrariesJar, LibrariesJarType}, loader::{forge::handler::ForgeLoaderVersionInfo, loader::{parse_group_relative_path, parse_group_url, LoaderArguments, LoaderJvmArgument, LoaderJvmArgumentType}}, util::{app_path, fetch, metadata}};

pub struct FabricHandler<'a> {
    minecraft_version: &'a str,
    loader_version: &'a str
}

impl<'a> FabricHandler<'a> {
    
    #[tracing::instrument]
    pub fn new(minecraft_version: &'a str, loader_version: &'a str) -> Self {
        FabricHandler {
            loader_version,
            minecraft_version
        }
    }

    pub async fn get_fabric_loader_version_info(&self) -> crate::Result<ForgeLoaderVersionInfo> {

        let fabric_loader_manifest = self.get_fabric_version_manifest().await?;
        let fabric_game_arguments = self.build_fabric_game_arguments(&fabric_loader_manifest)?;
        let fabric_jvm_arguments = self.build_fabric_jvm_arguments(&fabric_loader_manifest)?;
        let fabric_libraries = self.build_fabric_libraries(&fabric_loader_manifest.libraries)?;

        Ok(ForgeLoaderVersionInfo {
            loader_version: format!("{}-{}", "fabric", self.loader_version),
            main_class: fabric_loader_manifest.main_class.unwrap(),
            arguments: LoaderArguments {
                game: fabric_game_arguments,
                jvm: Some(fabric_jvm_arguments)
            },
            libraries: fabric_libraries,
            client_lzma: None,
            loader_install: None,
        })
    }

    fn build_fabric_libraries(&self, fabric_libraries: &Vec<daedalus::minecraft::Library>) -> crate::Result<Vec<LibrariesJar>> {

        let mut forge_libraries_result: Vec<LibrariesJar> = Vec::new();

        for fabric_lib in fabric_libraries.iter() {

            // ? ${modrinth.gameVersion} -> minecraft version ex: 1.20.2
            let name = &fabric_lib.name.replace("${modrinth.gameVersion}", &self.minecraft_version);

            // TODO: json key: downloads, If None skip
            if fabric_lib.url.is_none() {
                continue;
            }

            if let Some(url) = &fabric_lib.url {

                let url = parse_group_url(&url, &name)?;
                let file_path = parse_group_relative_path(&name)?;

                forge_libraries_result.push(LibrariesJar {
                    r#type: LibrariesJarType::ModLoader,
                    name: name.to_owned(),
                    relative_path: file_path.clone(),
                    path: app_path::combine_common_paths_absolute(Path::new("libraries"), &file_path),
                    sha1: "".to_owned(),
                    size: 0,
                    download_url: url,
                    relative_url: None,
                    manifest_url: None,
                    include_in_classpath: fabric_lib.include_in_classpath
                });
            }
        }

        Ok(forge_libraries_result)
    }

    #[tracing::instrument(skip(self, fabric_arguments))]
    fn build_fabric_jvm_arguments(&self, fabric_arguments: &daedalus::modded::PartialVersionInfo) -> crate::Result<Vec<LoaderJvmArgument>> {
        
        let mut fabric_jvm_arguments: Vec<LoaderJvmArgument> = Vec::new();

        let jvm_arguments = {
            let arguments = fabric_arguments.arguments.as_ref().ok_or_else(|| {
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
    
                                fabric_jvm_arguments.push(LoaderJvmArgument {
                                    r#type: LoaderJvmArgumentType::Space,
                                    name: name.to_owned(),
                                    value: jvm_value.to_owned().trim().to_owned(),
                                    keys: Some(variables),
                                })
                            } else {
                                
                                fabric_jvm_arguments.push(LoaderJvmArgument {
                                    r#type: LoaderJvmArgumentType::Space,
                                    name: name.to_owned(),
                                    value: jvm_value.to_owned().trim().to_owned(),
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

                                fabric_jvm_arguments.push(LoaderJvmArgument {
                                    r#type: LoaderJvmArgumentType::Equal,
                                    name: split[0].to_owned(),
                                    value: split[1].to_owned().trim().to_owned(),
                                    keys: Some(variables),
                                })

                            } else {
                                fabric_jvm_arguments.push(LoaderJvmArgument {
                                    r#type: LoaderJvmArgumentType::Equal,
                                    name: split[0].to_owned(),
                                    value: split[1].to_owned().trim().to_owned(),
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

        tracing::debug!("Fabric jvm arguments: {:#?}", fabric_jvm_arguments);

        Ok(fabric_jvm_arguments)
    }

    // 處理 Fabric 遊戲參數
    #[tracing::instrument(skip(self, fabric_arguments))]
    fn build_fabric_game_arguments(&self, fabric_arguments: &daedalus::modded::PartialVersionInfo) -> crate::Result<Vec<Argument>> {

        let mut fabric_game_arguments: Vec<Argument> = Vec::new();

        let fabric_arguments = {
            let arguments = fabric_arguments.arguments.as_ref().ok_or_else(|| {
                crate::ErrorKind::LoaderError("TODO".to_owned())
            })?;
            let game_arguments = arguments.get(&daedalus::minecraft::ArgumentType::Game).ok_or_else(|| {
                crate::ErrorKind::LoaderError("TODO".to_owned())
            })?;
            game_arguments
        };

        fabric_game_arguments = {

            let mut fabric_game_arguments: Vec<Argument> = Vec::new();

            let mut name = "".to_owned();
            let regex_1 = Regex::new(r"\$\{[^}]*\}").unwrap();
            let regex_2 = Regex::new(r"^--").unwrap();

            for forge_argument in fabric_arguments.iter() {
                match forge_argument {
                    daedalus::minecraft::Argument::Normal(value) => {

                        if regex_1.is_match(value) {
                            fabric_game_arguments.push(Argument {
                                name: name.clone(),
                                key: value.to_string(),
                                value: "".to_owned()
                            });
                        } else if regex_2.is_match(value) {
                            name = value.to_string();
                        } else {
                            fabric_game_arguments.push(Argument {
                                name: name.clone(),
                                key: "".to_owned(),
                                value: value.to_string()
                            });
                        }

                    },
                    _ => tracing::warn!("TODO Loader rules")
                }
            }
            fabric_game_arguments
        };

        tracing::debug!("Fabric game arguments: {:#?}", fabric_game_arguments);

        Ok(fabric_game_arguments)
    }

    // 提取獲取 Fabric 版本信息
    #[tracing::instrument(skip(self))]
    async fn get_fabric_version_manifest(&self) -> crate::Result<daedalus::modded::PartialVersionInfo> {
        let fabric_manifest = metadata::get_fabric_versions_manifest().await?;
        let modloaders = &fabric_manifest.game_versions[0].loaders;
        let fabric_loader_version = modloaders.iter().find(|v| v.id == self.loader_version)
            .ok_or_else(|| crate::ErrorKind::LoaderError(String::from("獲取模組載入器資料失敗，無法取得 Loader version")))?;
        Ok(fetch::request_json::<daedalus::modded::PartialVersionInfo>(&fabric_loader_version.url).await?)
    }
}