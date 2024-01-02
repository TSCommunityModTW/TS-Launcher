use allay_core::{init_logger, get_ts_launcher_assets};

const MINECRAFT_VERSION: &str = "1.20.2";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let _logger_guard = init_logger();

    let ts_launcher_assets = get_ts_launcher_assets().await?;

    println!("{:#?}", ts_launcher_assets);

    // println!("{:#?}", get_max_memory_size()?);

    // let java_path = install(17).await?;
    // println!("{:#?}", java::check_java_path(Path::new("/Library/Java/JavaVirtualMachines/jdk-17.0.1.jdk/Contents/Home/bin/java")).await);
    
    // let store = Store::get_write().await?;
    // store.settings.write().await.language = "????".to_owned();
    // let settings_path = app_path::get_settings_json_file_path();
    // store.settings.write().await.sync(&settings_path).await?;

    // let instance = store.instances.write().await.insert_new_instance("55555", InstanceJson {
    //     id: "55555".to_owned(),
    //     minecraft_version: String::from("0.0.0"),
    //     modpack: Modpack {
    //         name: String::from(""),
    //         version: String::from("0.0.0"),
    //         project_id: 0,
    //         file_id: 0,
    //         files: Vec::new(),
    //     },
    //     mod_loader: ModLoader {
    //         r#type: String::from(""),
    //         id: String::from(""),
    //         version: String::from("0.0.0"),
    //     },
    //     module: Module {
    //         size: 0,
    //         modules: Vec::new(),
    //     },
    // })?;

    // let instances_path = app_path::get_instances_json_file_path();

    // if let Some(instances) = store.instances.write().await.get("55555") {
    //     instances.write().await.minecraft_version = "5.0.0".to_owned();
    // }

    // store.instances.write().await.sync(&instances_path).await?;

    // let device_auth = microsoft::auth_device_code().await?;
    // let microsoft_token_auth = microsoft::auth_verification_user(device_auth).await?;
    // let minecraft_auth = minecraft::auth_minecraft(&microsoft_token_auth.access_token).await?;
    // println!("{:#?}", minecraft_auth);

    // match start_game(MINECRAFT_VERSION).await {
    //     Ok(_) => tracing::info!("Game started Ok"),
    //     Err(e) => tracing::error!(e),
    // }

    Ok(())
}

async fn start_game(minecraft_version: &str) -> Result<(), Box<dyn std::error::Error>> {

    // let java_jvm_path = "/Library/Java/JavaVirtualMachines/jdk-17.0.1.jdk/Contents/Home/bin/java";
    // let java_jvm_path = "/Library/Java/JavaVirtualMachines/zulu-17.jdk/Contents/Home/bin/java";
    // let java_jvm_path = "/Library/Java/JavaVirtualMachines/jdk1.8.0_311.jdk/Contents/Home/bin/java";
    // let java_jvm_path = "/Users/quasi-pc/Downloads/jre-17.0.9.jre/bin/java";
    // let java_jvm_path = "/Users/quasi-pc/Downloads/jdk-16.0.2.jdk/Contents/Home/bin/java";

    // let java_jvm_path = install(17).await?.to_string_lossy().to_string();

    // let mut children = Children::new();
    // children.rescue_cache().await?;

    // let vanilla_version_info = metadata::get_vanilla_version_info(minecraft_version).await?;

    // ! Forge 暫時不支援
    // let loader_version_info = BuildModLoader::new("1.5.2", LoaderType::Forge, "7.8.1.738", &vanilla_version_info).get_loader_version_info().await?;
    // let loader_version_info = BuildModLoader::new("1.6.4", LoaderType::Forge, "9.11.1.1345", &vanilla_version_info).get_loader_version_info().await?;

    // ? Forge 支援，已測試過
    // let loader_version_info = BuildModLoader::new("1.8.9", LoaderType::Forge, "11.15.1.2318-1.8.9", &vanilla_version_info).get_loader_version_info().await?;
    // let loader_version_info = BuildModLoader::new("1.7.10", LoaderType::Forge, "10.13.4.1614-1.7.10", &vanilla_version_info).get_loader_version_info().await?;
    // let loader_version_info = BuildModLoader::new("1.12.2", LoaderType::Forge, "14.23.5.2847", &vanilla_version_info).get_loader_version_info().await?;
    // let loader_version_info = BuildModLoader::new("1.12.2", LoaderType::Forge, "14.23.5.2860", &vanilla_version_info).get_loader_version_info().await?;
    // let loader_version_info = BuildModLoader::new("1.15.2", LoaderType::Forge, "31.2.50", &vanilla_version_info).get_loader_version_info().await?;
    // let loader_version_info = BuildModLoader::new("1.16.5", LoaderType::Forge, "36.2.34", &vanilla_version_info).get_loader_version_info().await?;
   
    // ! TODO: 1.19.2 , 1.18.2, 1.17.1 mac error
    // let loader_version_info = BuildModLoader::new("1.17.1", LoaderType::Forge, "37.1.1", &vanilla_version_info).get_loader_version_info().await?;
    // let loader_version_info = BuildModLoader::new("1.18.2", LoaderType::Forge, "40.2.14", &vanilla_version_info).get_loader_version_info().await?;
    // let loader_version_info = BuildModLoader::new("1.19.4", LoaderType::Forge, "45.2.6", &vanilla_version_info).get_loader_version_info().await?;

    // let loader_version_info = BuildModLoader::new("1.20.2", LoaderType::Forge, "48.0.40", &vanilla_version_info).get_loader_version_info().await?;

    // println!("{:#?}", loader_version_info);

    // validate::validate_installer(&vanilla_version_info, Some(&loader_version_info), Some(&java_jvm_path)).await?;
    // validate::validate_installer(&vanilla_version_info, None, Some(&java_jvm_path)).await?;
    
    // * modloader
    // let java_jvm_parameters = BuildParameters::new(&vanilla_version_info).get_jvm_loader_parameters(&loader_version_info)?;
    // * vanilla
    // let java_jvm_parameters = BuildParameters::new(&vanilla_version_info).get_jvm_vanilla_parameters()?;

    // tracing::info!("{:#?}", java_jvm_parameters.parameters.join(" "));
    // tracing::info!("{:#?}", java_jvm_parameters.parameters);

    // libraries::extract_natives(vanilla_version_info.get_libraries(), &java_jvm_parameters.natives_dir_path)?;

    // let mut child = Command::new(java_jvm_path);
    // child.args(&java_jvm_parameters.parameters);
    // child.current_dir(app_path::get_instances_dir_path().join("main-server"));

    // let _minecrafts_child = children.insert_new_process(Uuid::new_v4(), child).await?;

    // tokio::time::sleep(tokio::time::Duration::from_secs(500)).await;

    Ok(())
}