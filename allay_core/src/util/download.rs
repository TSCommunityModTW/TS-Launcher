use crate::{ErrorKind, util, LoadingBarId, emit::loading_try_for_each_concurrent};
use std::{fs::File, io::copy, path::{Path, PathBuf}, time::Duration};
use reqwest::{Client, Method};
use futures::{StreamExt, stream};
use sha1::{Sha1, Digest};

use super::{io::write_file, config::FETCH_ATTEMPTS};

lazy_static! {
    pub static ref REQWEST_CLIENT: reqwest::Client = {
        Client::builder()
            // .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build reqwest client")
    };
}

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

#[tracing::instrument(skip_all)]
pub async fn validate_download_assets(files: Vec<DownloadFile>, limit: Option<usize>, loading_bar: Option<(&LoadingBarId, f64)>) -> crate::Result<()> {

    let num_futs = files.len();
    let files = stream::iter(files.iter()).map(Ok::<&DownloadFile, crate::Error>);

    let mut loading = None;
    let mut loading_total = 0.0;

    if let Some((loading_bar_id, total)) = loading_bar {
        loading = Some(loading_bar_id);
        loading_total = total;
    }

    loading_try_for_each_concurrent(files, limit, loading, loading_total, num_futs, None,
        |file| async move {

            if !util::io::is_path_exists(&file.path) {
                tracing::debug!("本機檔案不存在，準備下載: {}", &file.name);
                match download_file(&file.download_url, &file.path, &file.name, &file.sha1, file.relative_url.as_ref(), file.manifest_url.as_ref()).await {
                    Ok(_) => tracing::debug!("檔案下載完成: {:?}", &file.path),
                    Err(error) => return Err(error),
                }
            }
    
            Ok(())
        }
    ).await?;

    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn download_file(url: &str, path: &Path, name: &str, sha1: &str, relative_url: Option<&String>, manifest_urls: Option<&Vec<String>>) -> crate::Result<()> {

    for attempt in 1..=(FETCH_ATTEMPTS + 2) {

        let response = REQWEST_CLIENT.request(Method::GET, url);
        let result = response.send().await;

        match result {
            Ok(mut res) => {

                if !res.status().is_success() {

                    tracing::warn!("取得下載檔案失敗，狀態碼: {}，嘗試重新取得，嘗試次數: {}", res.status(), attempt);
        
                    if let Some(manifest_urls) = manifest_urls {
        
                        tracing::warn!("嘗試使用備用清單 URL 列表取得, {}", url);
        
                        let relative_url = relative_url.ok_or_else(|| {
                            crate::ErrorKind::DownloadFileError("relative_url not None".to_owned())
                        })?;
        
                        let mut status: bool = false;
        
                        for manifest_url in manifest_urls.iter() {
        
                            let url = format!("{}{}", manifest_url, relative_url);
                            let backup_response = REQWEST_CLIENT.request(Method::GET, &url);
                            let result = backup_response.send().await;

                            match result {
                                Ok(backup_res) => {

                                    if backup_res.status().is_success() {
                                        tracing::warn!("成功使用備用清單 URL 列表取得下載檔案 {}", &url);
                                        res = backup_res;
                                        status = true;
                                        break;
                                    }

                                },
                                Err(err) => {
                                    if attempt <= 3 {
                                        continue;
                                    } else {
                                        return Err(err.into());
                                    }
                                }
                            }

                        }
        
                        if !status {
                            if attempt <= 3 {
                                continue;
                            } else {
                                return Err(crate::ErrorKind::DownloadFileError(format!("嘗試使用備用清單重新取得失敗: {}", url)).as_error());
                            }
                        }
        
                    } else if attempt <= 3 {
                        continue;
                    } else {
                        return Err(crate::ErrorKind::DownloadFileError(format!("嘗試重新取得失敗, url: {}", url)).as_error());
                    }
                }

                let bytes = res.bytes().await?;
                tracing::info!("下載成功，檔案大小: {} bytes", bytes.len());
                write_file(path, &bytes).await?;

                // 檢查下载文件的 SHA-1 哈希值是否匹配
                if !sha1_exists(path, sha1)? {

                    util::io::remove_file(path).await?;
                    tracing::warn!("已刪除不匹配的檔案: {:?}", path.to_string_lossy().to_owned());

                    if attempt <= 3 {
                        tracing::warn!("檔案 SHA-1 雜湊值不符，嘗試重新取得 name: {} sha1: {} url: {}", name.to_owned(), sha1, url.to_owned());
                        continue;
                    } else {
                        // tracing::warn!("嘗試重新失敗，先跳過, url: {}", url);
                        return Err(ErrorKind::FileSHA1Error(name.to_owned(), sha1.to_owned(), url.to_owned()).as_error());
                    }
                }

                return Ok(());
            },
            Err(err) => {
                if attempt <= 3 {
                    continue;
                } else {
                    return Err(err.into());
                }
            }
        }
    }

    Err(crate::ErrorKind::DownloadFileError(format!("下載檔案失敗, url: {}", url)).as_error())
}

// 檢查本地檔案的 SHA-1 雜湊值是否匹配
#[tracing::instrument(skip_all)]
pub fn sha1_exists(path: &Path, expected_hash: &str) -> crate::Result<bool> {

    let expected_hash = expected_hash.trim();

    if expected_hash.is_empty() {
        return Ok(true);
    }

    let mut local_file = File::open(path)?;
    let metadata = local_file.metadata()?;

    // 檢查檔案是否為空
    if metadata.len() == 0 {
        tracing::warn!("本地檔案為空，無法計算 SHA-1");
        return Ok(false);
    }

    let mut hasher = Sha1::new();
    copy(&mut local_file, &mut hasher)?;
    let binding = format!("{:x}", hasher.finalize()).to_owned();
    let local_hash = binding.trim();

    tracing::debug!("local: {} expected: {}", local_hash, expected_hash);

    Ok(local_hash == expected_hash)
}