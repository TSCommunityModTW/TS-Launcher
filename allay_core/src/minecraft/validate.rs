use std::path::Path;

use crate::{assets::game_assets::{IAssetsVersionMenifest, IAssetsVersionMenifestFile}, minecraft::loader::{forge::installer::ForgeInstaller, loader::LoaderType}, util::{config, download::{self, DownloadFile}}, LoadingBarId};
use super::{version::{VanillaVersionInfo, ClientJar}, loader::loader::LoaderVersionInfo, libraries::LibrariesJar, assets::AssetObjects};


#[tracing::instrument(skip_all)]
pub async fn validate_installer(vanilla_version_info: &VanillaVersionInfo, loader_version_info: Option<&LoaderVersionInfo>, game_assets_version_menifest: Option<&IAssetsVersionMenifest>, game_dir_path: &Path, java_jvm_path: Option<&str>, loading_bar: Option<(&LoadingBarId, f64)>) -> crate::Result<()> {

    tracing::info!("Validate downloading Data...");
    tracing::info!("Add vanilla downloads queue.");

    let download_queue: Vec<DownloadFile> = {

        let mut queue = Vec::new();

        // 添加客戶端 JAR 檔案到下載佇列
        add_client_jar_to_download_queue(vanilla_version_info.get_client_jar(), &mut queue);
        // 添加資產檔案到下載佇列
        add_asset_objects_to_download_queue(vanilla_version_info.get_asset_objects().await?, &mut queue);
        // 添加庫檔案到下載佇列
        add_libraries_to_download_queue(vanilla_version_info.get_libraries(), &mut queue);

        // 添加遊戲檔案資料到下載佇列
        if let Some(game_assets_version_menifest) = game_assets_version_menifest {
            add_game_assets_file_to_download_queue(&game_assets_version_menifest.files, game_dir_path, &mut queue);
        }
        
        // loader
        if let Some(loader_version_info) = loader_version_info {

            tracing::info!("Add loader downloads queue");
    
            // 添加庫檔案到下載佇列
            add_loader_libraries_to_download_queue(&loader_version_info.libraries, &mut queue);

            if loader_version_info.r#type == LoaderType::Forge {
                // 添加 client_lzma 檔案到下載佇列
                add_client_lzma_to_download_queue(&loader_version_info.forge.as_ref().unwrap().client_lzma, &mut queue);
            }
        }

        queue
    };

    // 驗證下載的資產
    download::validate_download_assets(download_queue, Some(config::APP_DOWNLOAD_LIMIT), loading_bar).await?;

    // forge modloader installer
    if let Some(loader_version_info) = loader_version_info {
        if loader_version_info.r#type == LoaderType::Forge && loader_version_info.forge.as_ref().unwrap().loader_install.is_some() {

            let loader_install = loader_version_info.forge.as_ref().unwrap().loader_install.as_ref().unwrap();
            let vanilla_client_jar = &vanilla_version_info.get_client_jar();
            let forge_installer = ForgeInstaller::new(&loader_install.data, vanilla_client_jar);

            let java_jvm_path = java_jvm_path.unwrap();

            forge_installer.install(&loader_install.processors, java_jvm_path).await?;
        }
    }

    tracing::info!("Done downloading Data!");

    Ok(())
}

#[tracing::instrument(skip_all)]
fn add_game_assets_file_to_download_queue(files: &Vec<IAssetsVersionMenifestFile>, game_dir_path: &Path, queue: &mut Vec<DownloadFile>) {
    for file in files.iter() {
        let path = game_dir_path.join(&file.install_path);
        queue.push(DownloadFile {
            name: file.name.to_owned(),
            path: path,
            sha1: file.hash.to_owned(),
            size: file.size,
            download_url: file.download_url.to_owned(),
            relative_url: None,
            manifest_url: None
        });
    }
}
 
#[tracing::instrument(skip_all)]
fn add_libraries_to_download_queue(libraries: Vec<LibrariesJar>, queue: &mut Vec<DownloadFile>) {
    for lib in libraries.iter() {
        queue.push(DownloadFile {
            name: lib.name.to_owned(),
            path: lib.path.to_path_buf(),
            sha1: lib.sha1.to_owned(),
            size: lib.size,
            download_url: lib.download_url.to_owned(),
            relative_url: lib.relative_url.to_owned(),
            manifest_url: lib.manifest_url.to_owned()
        });
    }
}

#[tracing::instrument(skip_all)]
fn add_asset_objects_to_download_queue(asset_objects: Vec<AssetObjects>, queue: &mut Vec<DownloadFile>) {
    for obj in asset_objects.iter() {
        queue.push(DownloadFile {
            name: obj.name.to_owned(),
            path: obj.path.to_path_buf(),
            sha1: obj.sha1.to_owned(),
            size: obj.size,
            download_url: obj.download_url.to_owned(),
            relative_url: None,
            manifest_url: None
        });
    }
}

#[tracing::instrument(skip_all)]
fn add_client_jar_to_download_queue(client_jar: ClientJar, queue: &mut Vec<DownloadFile>) {
    queue.push(DownloadFile {
        name: client_jar.name,
        path: client_jar.path,
        sha1: client_jar.sha1,
        size: client_jar.size,
        download_url: client_jar.download_url,
        relative_url: None,
        manifest_url: None
    });
}

fn add_client_lzma_to_download_queue(client_lzma: &Option<LibrariesJar>, queue: &mut Vec<DownloadFile>)  {

    // ? If None to skip
    let client_lzma = match client_lzma {
        Some(v) => v,
        None => return,
    };

    queue.push(DownloadFile {
        name: client_lzma.name.to_owned(),
        path: client_lzma.path.to_path_buf(),
        sha1: client_lzma.sha1.to_owned(),
        size: client_lzma.size,
        download_url: client_lzma.download_url.to_owned(),
        relative_url: None,
        manifest_url: None,
    });
}

fn add_loader_libraries_to_download_queue(libraries: &Vec<LibrariesJar>, queue: &mut Vec<DownloadFile>) {
    for lib in libraries.iter() {
        queue.push(DownloadFile {
            name: lib.name.to_owned(),
            path: lib.path.to_path_buf(),
            sha1: lib.sha1.to_owned(),
            size: lib.size,
            download_url: lib.download_url.to_owned(),
            relative_url: lib.relative_url.to_owned(),
            manifest_url: lib.manifest_url.to_owned()
        });
    }
}