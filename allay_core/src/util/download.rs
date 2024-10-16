use crate::{ErrorKind, util, LoadingBarId, emit::loading_try_for_each_concurrent};
use std::{fs::File, io::copy, path::{Path, PathBuf}, time::Duration};
use reqwest::{Client, Method};
use futures::{StreamExt, stream};
use sha1::{Sha1, Digest};

use super::{io::write_file, config::FETCH_ATTEMPTS};

lazy_static! {
    pub static ref REQWEST_CLIENT: reqwest::Client = {
        Client::builder()
            .timeout(Duration::from_secs(30)) // 增加整體請求超時
            .connect_timeout(Duration::from_secs(10)) // 設定連接超時
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
    tracing::debug!("開始驗證下載資源，共 {} 個檔案", num_futs);

    let files = stream::iter(files.iter()).map(Ok::<&DownloadFile, crate::Error>);

    let mut loading = None;
    let mut loading_total = 0.0;

    if let Some((loading_bar_id, total)) = loading_bar {
        tracing::debug!("設定進度條, ID: {:?}, 總量: {}", loading_bar_id, total);
        loading = Some(loading_bar_id);
        loading_total = total;
    }

    loading_try_for_each_concurrent(files, limit, loading, loading_total, num_futs, None,
        |file| async move {

            tracing::debug!("正在處理檔案: {} (SHA1: {})", file.name, file.sha1);

            if !util::io::is_path_exists(&file.path) {
                tracing::debug!("本機檔案不存在，準備下載: {}", &file.name);
                match download_file(&file.download_url, &file.path, &file.name, &file.sha1, file.relative_url.as_ref(), file.manifest_url.as_ref()).await {
                    Ok(_) => tracing::debug!("檔案下載完成: {:?}", &file.path),
                    Err(error) => {
                        tracing::error!("下載檔案失敗: {} (錯誤: {:?})", file.name, error);
                        return  Err(error);
                    },
                }
            } else {
                tracing::debug!("本機檔案已存在: {}", &file.name);
            }
    
            Ok(())
        }
    ).await?;

    tracing::debug!("所有資源驗證及下載已完成");

    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn download_file(url: &str, path: &Path, name: &str, sha1: &str, relative_url: Option<&String>, manifest_urls: Option<&Vec<String>>) -> crate::Result<()> {

    tracing::debug!("開始下載檔案: {} (URL: {})", name, url);

    for attempt in 1..=(FETCH_ATTEMPTS + 2) {

        tracing::debug!("下載次數: {}，URL: {}", attempt, url);

        let response = REQWEST_CLIENT.request(Method::GET, url);
        let result = response.send().await;

        match result {
            Ok(mut res) => {

                tracing::debug!("收到回應, 狀態碼: {}", res.status());

                if !res.status().is_success() {

                    tracing::warn!("下載失敗，狀態碼: {}，嘗試重新取得，嘗試次數: {}", res.status(), attempt);
        
                    if let Some(manifest_urls) = manifest_urls {
        
                        tracing::warn!("嘗試使用備用清單 URL 列表取得, URL: {}", url);
        
                        let relative_url = relative_url.ok_or_else(|| {
                            crate::ErrorKind::DownloadFileError("relative_url not None".to_owned())
                        })?;
        
                        let mut status: bool = false;
        
                        for manifest_url in manifest_urls.iter() {
        
                            let url = format!("{}{}", manifest_url, relative_url);
                            tracing::debug!("嘗試備用 URL: {}", url);
                            let backup_response = REQWEST_CLIENT.request(Method::GET, &url);
                            let result = backup_response.send().await;

                            match result {
                                Ok(backup_res) => {

                                    if backup_res.status().is_success() {
                                        tracing::debug!("成功使用備用 URL 列表下載檔案: {}", &url);
                                        res = backup_res;
                                        status = true;
                                        break;
                                    }

                                },
                                Err(err) => {
                                    tracing::error!("備用 URL 下載失敗: {} (錯誤: {:?})", url, err);
                                    if attempt <= 3 {
                                        continue;
                                    } else {
                                        return Err(err.into());
                                    }
                                }
                            }

                        }
        
                        if !status {
                            tracing::error!("所有備用 URL 下載失敗: {}", url);
                            if attempt <= 3 {
                                continue;
                            } else {
                                return Err(crate::ErrorKind::DownloadFileError(format!("嘗試使用備用清單重新取得失敗: {}", url)).as_error());
                            }
                        }
        
                    } else if attempt <= 3 {
                        continue;
                    } else {
                        return Err(crate::ErrorKind::DownloadFileError(format!("嘗試重新取得失敗, URL: {}", url)).as_error());
                    }
                }

                let bytes = res.bytes().await?;
                tracing::info!("下載成功，檔案大小: {} bytes", bytes.len());
                write_file(path, &bytes).await?;

                // 檢查下载文件的 SHA-1 哈希值是否匹配
                if !sha1_exists(path, sha1)? {

                    util::io::remove_file(path).await?;
                    tracing::debug!("已刪除不匹配的檔案: {:?}", path.to_string_lossy().to_owned());

                    if attempt <= 3 {
                        tracing::debug!("檔案雜湊值不符，嘗試重新取得 Name: {} SHA-1: {} URL: {}", name.to_owned(), sha1, url.to_owned());
                        continue;
                    } else {
                        // tracing::warn!("嘗試重新失敗，先跳過, url: {}", url);
                        return Err(ErrorKind::FileSHA1Error(name.to_owned(), sha1.to_owned(), url.to_owned()).as_error());
                    }
                }

                return Ok(());
            },
            Err(err) => {
                tracing::error!("下載失敗, URL: {} (錯誤: {:?})", url, err);
                if attempt <= 3 {
                    continue;
                } else {
                    return Err(err.into());
                }
            }
        }
    }

    tracing::error!("下載失敗，已超過最大嘗試次數, URL: {}", url);
    Err(crate::ErrorKind::DownloadFileError(format!("下載檔案失敗, url: {}", url)).as_error())
}

// 檢查本地檔案的 SHA-1 雜湊值是否匹配
#[tracing::instrument(skip_all)]
pub fn sha1_exists(path: &Path, expected_hash: &str) -> crate::Result<bool> {

    tracing::debug!("開始檢查檔案的 SHA-1 雜湊值，預期雜湊值: {}", expected_hash); 

    let expected_hash = expected_hash.trim();

    if expected_hash.is_empty() {
        tracing::debug!("預期雜湊值為空，視為匹配");
        return Ok(true);
    }

    let mut local_file = File::open(path)?;
    let metadata = local_file.metadata()?;

    // 檢查檔案是否為空
    if metadata.len() == 0 {
        tracing::debug!("本地檔案為空，無法計算 SHA-1");
        return Ok(false);
    }

    let mut hasher = Sha1::new();
    copy(&mut local_file, &mut hasher)?;
    let binding = format!("{:x}", hasher.finalize()).to_owned();
    let local_hash = binding.trim();

    tracing::debug!("本地檔案的雜湊值: {}，預期雜湊值: {}", local_hash, expected_hash);

    Ok(local_hash == expected_hash)
}