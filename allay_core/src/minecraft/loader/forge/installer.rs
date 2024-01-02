use std::{collections::HashMap, path::Path};

use daedalus::modded::{SidedDataEntry, Processor};
use regex::Regex;
use tokio::process::Command;

use crate::{minecraft::{version::ClientJar, loader::loader}, util::{download::sha1_exists, io, app_path, utils::{OSType, self}}};

/// Regex: --
const REGEX_1: &str = r"^--";
/// Regex: [[]]
const REGEX_2: &str = r"\[[^}]*\]";
/// Regex: {}
const REGEX_3: &str = r"\{[^}]*\}";

#[derive(Debug)]
pub struct Argument {
    key: String,
    value: String
}

impl Argument {
    pub fn as_array(&self) -> Vec<String> {
        if self.value.is_empty() {
            vec![self.key.clone()]
        } else {
            vec![self.key.clone(), self.value.clone()]
        }
    }
}

#[derive(Debug)]
pub struct ForgeInstaller<'a> {
    data_key_map: &'a HashMap<String, SidedDataEntry>,
    minecraft_client_jar: &'a ClientJar,
}

impl<'a> ForgeInstaller<'a> {

    #[tracing::instrument()]
    pub fn new(data_key_map: &'a HashMap<String, SidedDataEntry>, minecraft_client_jar: &'a ClientJar) -> Self {
        ForgeInstaller {
            data_key_map,
            minecraft_client_jar,
        }
    }

    #[tracing::instrument(skip(self, processors, java_jvm_path))]
    pub async fn install(&self, processors: &Vec<Processor>, java_jvm_path: &str) -> crate::Result<()> {

        if !self.is_install()? {
            return Ok(());
        }

        tracing::info!("Install forge modloader...");

        for processor in processors.iter() {

            // ? If sides client ok, no server
            if processor.sides.is_some() {
                let sides = processor.sides.as_ref().unwrap();
                let is_client_side = sides.iter().find(|v| v.contains("client"));
                if is_client_side.is_none() {
                    continue;
                }
            }

            let main_class = self.parse_main_class(&processor.jar);
            let classpath = self.get_classpath(&processor.classpath, &processor.jar)?;

            let args = &processor.args;

            let regex_1 = Regex::new(REGEX_1).unwrap();
            let regex_2 = Regex::new(REGEX_2).unwrap();
            let regex_3 = Regex::new(REGEX_3).unwrap();

            let mut arguments: Vec<Argument> = Vec::new();
            for (i, value) in args.iter().enumerate() {

                // let mut val = "".to_owned();

                // println!("{:#?} {:#?}", value, !regex_1.is_match(&value));
                // println!("{:#?} {:#?} {:#?} {:#?}", value, i + 1, args.len(), (i + 1) < args.len());

                if !regex_1.is_match(&value) {

                    let mut val = match value {
                        _ => value.to_owned()
                    };

                    if regex_2.is_match(&value) || regex_3.is_match(&value) {
                        val = self.get_data_key_value(&value)?;
                    }

                    arguments.push(Argument {
                        key: args[i - 1].to_string(),
                        value: val
                    });
                } else {
                    
                    // * 處理 Forge >= 37.0.0
                    {
                        if (i + 1) < args.len() && !regex_1.is_match(&args[i + 1]) {
                            continue;
                        } else if !regex_1.is_match(&value) {
                            continue;
                        }
    
                        arguments.push(Argument {
                            key: value.to_owned(),
                            value: "".to_owned()
                        });
                    }

                }
            }

            let arguments: Vec<String> = arguments
                .iter()
                .flat_map(|parameter| parameter.as_array())
                .collect();

            // Add parameters
            let mut parameters: Vec<String> = Vec::new();
            parameters.push("-cp".to_owned());
            parameters.push(classpath);
            parameters.push(main_class.to_owned());
            parameters.extend(arguments);

            tracing::debug!("{:#?}", parameters);

            self.child(&java_jvm_path, &parameters).await?;
        }

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    fn is_install(&self) -> crate::Result<bool> {
        
        tracing::info!("Forge exists is install...");

        let exists_file = |key: &str, sha1: &str| -> crate::Result<bool> {

            let path_str= self.get_data_key_value(&key)?;
            let path= Path::new(&path_str);

            if io::is_path_exists(&path) && sha1_exists(Path::new(&path), &sha1)? {
                tracing::info!("Forge exists [{}] sha1 ok.", path.display());
                return Ok(true);
            } else {
                tracing::warn!("Forge exists [{}] sha1 error, expected: {}", path.display(), sha1);
                return Ok(false);
            }
        };

        let mc_slim = self.data_key_map.get("MC_SLIM");
        let mc_slim_sha = self.data_key_map.get("MC_SLIM_SHA");

        if mc_slim.is_some() && mc_slim_sha.is_some() && !exists_file(&mc_slim.unwrap().client, &mc_slim_sha.unwrap().client.replace("'", ""))? {
            return Ok(true);
        }

        let mc_extra = self.data_key_map.get("MC_EXTRA");
        let mc_extra_sha = self.data_key_map.get("MC_EXTRA_SHA");

        if mc_extra.is_some() && mc_extra_sha.is_some() && !exists_file(&mc_extra.unwrap().client, &mc_extra_sha.unwrap().client.replace("'", ""))? {
            return Ok(true);
        }

        let patched = self.data_key_map.get("PATCHED");
        let patched_sha = self.data_key_map.get("PATCHED_SHA");

        if patched.is_some() && patched_sha.is_some() && !exists_file(&patched.unwrap().client, &patched_sha.unwrap().client.replace("'", ""))? {
            return Ok(true);
        }

        tracing::info!("Forge all exists is install sha1 ok.");

        return Ok(false);
    }

    #[tracing::instrument(skip(self))]
    fn get_data_key_value(&self, key: &str) -> crate::Result<String> {

        let libraries_dir_path = app_path::get_common_dir_path().join("libraries");

        // * if {}
        if Regex::new(REGEX_3).unwrap().is_match(key) {

            let key = Regex::new(r"^\{|\}$").unwrap().replace_all(key, "").to_string();

            if key == "MINECRAFT_JAR" {
                return Ok(self.minecraft_client_jar.path.to_string_lossy().to_string());
            }

            // * 處理 Forge >= 37.0.0
            if key == "SIDE" {
                return Ok("client".to_owned());
            }

            // println!("{}", key);

            let client_data_key = &self.data_key_map.get(&key).ok_or_else(|| {
                crate::ErrorKind::LoaderError("Get [data_key_map] error".to_owned())
            })?.client;

            return Ok(self.get_data_key_value(client_data_key)?);
        }

        // * if []
        if Regex::new(REGEX_2).unwrap().is_match(key) {
            let key = Regex::new(r"^\[|\]$").unwrap().replace_all(key, "").to_string();
            return Ok(libraries_dir_path.join(loader::parse_group_relative_path(&key)?).to_string_lossy().to_string());
        }

        Err(crate::ErrorKind::LoaderError("Get data key value error".to_owned()).as_error())
    }

    #[tracing::instrument(skip(self, java_jvm_path, parameters))]
    async fn child(&self, java_jvm_path: &str, parameters: &Vec<String>) -> crate::Result<()> {

        let mut child = Command::new(java_jvm_path)
            .args(parameters)
            .spawn()?;

        match child.try_wait() {
            Ok(Some(status)) => tracing::info!("Forge installer Exited with: {}", status),
            Ok(None) => {
                tracing::info!("Run forge installer child.");
                let res = child.wait().await?;
                if res.success() {
                    tracing::info!("Forge installer Result: {}", res);
                } else {
                    return Err(crate::ErrorKind::LoaderError(res.to_string()).as_error());
                }
            }
            Err(e) => {
                return Err(crate::ErrorKind::LoaderError(e.to_string()).as_error());
            },
        }

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    fn parse_main_class(&self, value: &str) -> &str {
        // * net.minecraftforge:installertools:1.2.6 -> Get installertools
        let split = value.split(":").collect::<Vec<&str>>();
        match split[1] {
            "installertools" => "net.minecraftforge.installertools.ConsoleTool",
            "jarsplitter" => "net.minecraftforge.jarsplitter.ConsoleTool",
            "SpecialSource" => "net.md_5.specialsource.SpecialSource",
            "binarypatcher" => "net.minecraftforge.binarypatcher.ConsoleTool",
            "ForgeAutoRenamingTool" => "net.minecraftforge.fart.Main",
            "vignette" => "org.cadixdev.vignette.VignetteMain",
            _ => panic!("Parse jar main class Error.")
        }
    }

    #[tracing::instrument(skip(self))]
    fn get_classpath(&self, classpath: &Vec<String>, lib_tool_jar: &str) -> crate::Result<String> {

        let mut libraries_path = Vec::new();
        let libraries_dir_path = app_path::get_common_dir_path().join("libraries");

        libraries_path.push(libraries_dir_path.join(loader::parse_group_relative_path(&lib_tool_jar)?).to_string_lossy().to_string());

        for name in classpath.iter() {
            let relative_file_path = &loader::parse_group_relative_path(&name)?;
            libraries_path.push(libraries_dir_path.join(relative_file_path).to_string_lossy().to_string());
        }

        Ok(self.assemble_library_path(libraries_path))
    }

    #[tracing::instrument(skip(self))]
    fn assemble_library_path(&self, libraries_path: Vec<String>) -> String {

        let mut libraries: Vec<String> = Vec::new();

        for library in libraries_path.iter() {
            libraries.push(library.to_owned());
        }
        
        // 根據操作系統類型選擇路徑分隔符
        if utils::get_os_type() == OSType::Windows {
            libraries.join(";") // 在 Windows 系統中使用分號分隔，並回傳值
        } else {
            libraries.join(":") // 在非 Windows 系統中使用冒號分隔，並回傳值
        }
    }
}