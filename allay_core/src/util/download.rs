use crate::{ErrorKind, util};
use std::{path::{PathBuf, Path}, sync::{Arc, Mutex}, fs::File, io::{self, Write}};
use futures::StreamExt;
use reqwest::Client;
use sha1::{Sha1, Digest};

use super::io::create_dir_all;

#[derive(Debug, Clone)]
pub struct DownloadFile {
    pub name: String,
    pub path: PathBuf,
    pub sha1: String,
    pub size: u32,
    pub download_url: String,
    pub relative_url: Option<String>,
    pub manifest_url: Option<Vec<String>>,
}

// #[tokio::main]
#[tracing::instrument(skip(files))]
pub async fn validate_download_assets(files: Vec<DownloadFile>, limit: usize) -> crate::Result<()> {
    
    let files_len = &files.len();

    tracing::info!("Validate downloading game data total: {}", files_len);

    let mut download_tasks = Vec::new();
    let download_failure: Arc<Mutex<Vec<DownloadFile>>> = Arc::new(Mutex::new(Vec::new()));

    for file in files {

        let download_failure = Arc::clone(&download_failure);

        let file_name = file.name;
        let file_path = file.path;
        let file_sha1 = file.sha1;
        let file_size = file.size;
        let download_url = file.download_url;
        let relative_url = file.relative_url;
        let manifest_url = file.manifest_url;

        // if utils::is_path_exists(&file_path) {
        //     if sha1_exists(&file_path, sha1)? {
        //         // println!("Local file SHA-1 matches: {}", file_name);
        //         continue;
        //     } else {
        //         debug!("Local file SHA-1 does not match, expected: {}", file_name);
        //     }
        // }

        if util::io::is_path_exists(&file_path) { continue; }

        let handle = tokio::spawn(async move {

            tracing::debug!("Local file does not exist, ready to download: {}", file_name);
            
            match download_file(&download_url, &file_path, &file_name, &file_sha1, relative_url.as_ref(), manifest_url.as_ref()).await {
                Ok(_) => tracing::debug!("Downloaded file finish: {:?}", file_path),
                Err(error) => {

                    let mut download_failure = download_failure.lock().unwrap();
                    download_failure.push(DownloadFile {
                        name: file_name.clone(),
                        path: file_path,
                        sha1: file_sha1.clone(),
                        size: file_size,
                        download_url: download_url.clone(),
                        relative_url: relative_url,
                        manifest_url: manifest_url
                    });

                    tracing::error!("Download failure: {}", error);
                },
            }
        });

        download_tasks.push(handle);

        if download_tasks.len() >= limit {
            futures::future::try_join_all(download_tasks.drain(..)).await?;
        }
    }

    futures::future::try_join_all(&mut download_tasks).await?;

    let download_failure = download_failure.lock().unwrap();
    let download_failure_count = download_failure.len();
    tracing::info!("Validate downloading game data END! Success: {} Failure: {}", files_len - download_failure_count, download_failure_count);

    if download_failure_count > 0 {
        return Err(ErrorKind::DownloadFilesError(download_failure_count).as_error());
    }

    Ok(())
}

#[tracing::instrument(skip(url))]
pub async fn download_file(url: &str, path: &Path, name: &str, sha1: &str, relative_url: Option<&String>, manifest_urls: Option<&Vec<String>>) -> crate::Result<()> {

    // 檢查響應是否成功 error_for_status()
    // let response = Client::new().get(url).send().await?.error_for_status()?;

    let mut response = Client::new().get(url).send().await?;

    // 檢查響應是否成功 
    if !response.status().is_success() {

        if let Some(manifest_urls) = manifest_urls {

            tracing::warn!("取得下載檔案失敗，嘗試使用備用清單 URL 列表取得, {}", url);

            let relative_url = relative_url.ok_or_else(|| {
                crate::ErrorKind::DownloadFileError("relative_url not None".to_owned())
            })?;

            let mut status: bool = false;
            let mut status_manifest_url = "".to_owned();
            for manifest_url in manifest_urls.iter() {
                let url = format!("{}{}", manifest_url, relative_url);
                let res = Client::new().get(&url).send().await?;
                if res.status().is_success() {
                    tracing::warn!("成功使用備用清單 URL 列表取得下載檔案 {}", &url);
                    response = res;
                    status = true;
                    break;
                }
                status_manifest_url = url;
            }

            if !status {
                Client::new().get(&status_manifest_url).send().await?.error_for_status()?;
            }
        }
    }
    
    // create all dir 
    if let Some(parent) = path.parent() {
        create_dir_all(parent).await?;
    }

    let mut file = File::create(path).or(Err(ErrorKind::CreateFileIOError(path.to_string_lossy().to_string())))?;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(ErrorKind::DownloadFileError(name.to_string())))?;
        file.write_all(&chunk).or(Err(ErrorKind::DownloadFileError(name.to_string())))?;
    }

    // 檢查下载文件的 SHA-1 哈希值是否匹配
    if !sha1_exists(path, sha1)? {
        tracing::warn!("File SHA-1 hash does not match. {} sha1: {} url: {}", name.to_owned(), sha1, url.to_owned());
        return Err(ErrorKind::FileSHA1Error(name.to_owned(), sha1.to_owned(), url.to_owned()).as_error());
    }

    Ok(())
}

 // 檢查本地檔案的 SHA-1 雜湊值是否匹配
pub fn sha1_exists(path: &Path, expected_hash: &str) -> crate::Result<bool> {

    if expected_hash.is_empty() {
        return Ok(true);
    }

    let mut local_file = File::open(path)?;
    let mut hasher = Sha1::new();
    io::copy(&mut local_file, &mut hasher)?;
    let local_hash = format!("{:x}", hasher.finalize());
    Ok(local_hash == expected_hash.to_string())
}