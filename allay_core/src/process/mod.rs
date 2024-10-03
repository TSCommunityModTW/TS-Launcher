use tokio::process::Command;
use uuid::Uuid;

use crate::{assets::game_assets, data::LauncherAssets, emit::{emit_loading, init_loading}, launcher_assets::ServerChildren, minecraft::{libraries, loader::loader::{BuildModLoader, LoaderType}, parameters::{BuildParameters, JavaJvmSettings, PlayerProfile}, validate}, util::{app_path, io::create_dir_all, metadata, utils}, Java, LoadingBarType, Store};

pub mod children;

pub struct Process {}

impl Process {
    
    pub async fn run(server_id: &str, children_server_id: &str, uuid: Uuid) -> crate::Result<()> {

        let loading_bar = init_loading(
            LoadingBarType::ProcessChildren {
                server_id: children_server_id.to_owned(),
                id: uuid.to_string()
            },
            100.0,
            &format!(""),
        ).await?;

        let store = Store::get().await?;
        let settings = store.settings.read().await;
        let profiles = store.profiles.read().await;

        let global_java_settings = settings.java.get("global").ok_or_else(|| {
            crate::ErrorKind::from(crate::ErrorKind::LauncherError("無法獲取 Java 全域設置".to_owned()))
        })?;

        let children_server: ServerChildren;
    
        {
            if let Some(launcher_assets_server) = LauncherAssets::get_servers()?.iter().find(|server| server.id == server_id) {
                if let Some(launcher_assets_children_server) = launcher_assets_server.children.iter().find(|children_server| children_server.id == children_server_id) {
                    children_server = launcher_assets_children_server.clone();
                } else {
                    return Err(crate::ErrorKind::LauncherError(format!("找不到子伺服器的資產 server_id: {}, children_server_id: {}", server_id, children_server_id)).as_error());
                }
            } else {
                return Err(crate::ErrorKind::LauncherError(format!("找不到伺服器的資產 server_id: {}", server_id)).as_error());
            }
        }

        // TODO
        // let minecraft_version = children_server.minecraft_version;

        let game_dir_path = app_path::get_instances_dir_path().join(children_server_id);

        // 確保 Game 目錄路徑存在
        create_dir_all(&game_dir_path).await?;

        // Get game assets file
        let game_assets_version_menifest  = game_assets::get_game_assets_version_menifest(&server_id, &children_server_id, &children_server.assets.version).await?;

        emit_loading(&loading_bar, 5.0, None).await?;
        // Get vanilla assets
        let vanilla_version_info = metadata::get_vanilla_version_info(&game_assets_version_menifest.minecraft.version).await?;

        let loader_type: LoaderType = match game_assets_version_menifest.modloader.r#type.as_str() {
            "Forge" => LoaderType::Forge,
            "Fabric" => LoaderType::Fabric,
            _ => panic!("Not support loader.")
        };

        // Get loader assets
        let loader_version_info = BuildModLoader::new(&game_assets_version_menifest.minecraft.version, loader_type, &game_assets_version_menifest.modloader.version, &vanilla_version_info).get_loader_version_info().await?;

        let java_settings: Java;
        // Get java settings
        {
            if let Some(java) = settings.java.get(children_server_id) {
                java_settings = java.clone();
            } else {
                java_settings = global_java_settings.clone();
            }
        }

        let java_jvm_path: String;
        let java_parameter = &java_settings.java_parameter;
        let ram_max_size = &java_settings.ram_max_size;
        let ram_min_size = &java_settings.ram_min_size;

        {
            if java_settings.java_path_checked {
                java_jvm_path = java_settings.java_path;
            } else {
                java_jvm_path = Self::is_mc_java_jvm_path(&game_assets_version_menifest.minecraft.version, &global_java_settings);
            }
        }

        emit_loading(&loading_bar, 15.0, None).await?;
        validate::validate_installer(&vanilla_version_info, Some(&loader_version_info), Some(&game_assets_version_menifest), &game_dir_path, Some(&java_jvm_path), Some((&loading_bar, 60.0))).await?;
        emit_loading(&loading_bar, 15.0, None).await?;

        let java_jvm_settings = JavaJvmSettings {
            ram_max_size: ram_max_size.to_owned(),
            ram_min_size: ram_min_size.to_owned(),
            java_parameter: java_parameter.to_owned(),
            java_jvm_path: java_jvm_path.clone()
        };

        let player_profile = PlayerProfile {
            name: profiles.player.name.clone(),
            uuid: profiles.player.uuid.clone(),
            mc_account_token: profiles.microsoft_auth.mc_account_token.clone(),
        };

        // let java_jvm_parameters = BuildParameters::new(&vanilla_version_info, &game_dir_path, java_jvm_settings, player_profile).get_jvm_vanilla_parameters()?;
        let java_jvm_parameters = BuildParameters::new(&vanilla_version_info, &game_dir_path, java_jvm_settings, player_profile).get_jvm_loader_parameters(&loader_version_info)?;
        tracing::info!("{:#?}", java_jvm_parameters.parameters);

        let mut children = store.children.write().await;

        libraries::extract_natives(vanilla_version_info.get_libraries(), &java_jvm_parameters.natives_dir_path)?;
        emit_loading(&loading_bar, 5.0, None).await?;

        if java_jvm_path.is_empty() {
            return Err(crate::ErrorKind::LauncherError("Java 虛擬機路徑不能為空".to_owned()).as_error());
        }

        let mut child = Command::new(&java_jvm_path);
        child.args(&java_jvm_parameters.parameters);
        child.current_dir(&game_dir_path);

        let _minecrafts_child = children.insert_new_process(uuid, child).await?;

        Ok(())
    }

    fn is_mc_java_jvm_path(mc_version: &str, java: &Java) -> String {
        if utils::is_mc_version("1.18", mc_version) {
            return java.java17_path.clone();
        } else if utils::is_mc_version("1.17", mc_version) {
            return java.java16_path.clone();
        } else {
            java.java8_path.clone()
        }
    }
}