use crate::{config::FETCH_ATTEMPTS, emit::loading_try_for_each_concurrent, util, ErrorKind, LoadingBarId};
use std::{fs::File, io::copy, path::{Path, PathBuf}, time::Duration};
use reqwest::{Client, Method};
use futures::{StreamExt, stream};
use sha1::{Sha1, Digest};

lazy_static! {
    pub static ref REQWEST_CLIENT: reqwest::Client = {
        Client::builder()
            .timeout(Duration::from_secs(300)) // 增加整體請求超時
            .connect_timeout(Duration::from_secs(30)) // 設定連接超時
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
    tracing::info!("開始驗證下載資源，共 {} 個檔案", num_futs);

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
                tracing::info!("本機檔案不存在，準備下載: {}", &file.name);
                match download_file(&file.download_url, &file.path, &file.name, &file.sha1, file.relative_url.as_ref(), file.manifest_url.as_ref()).await {
                    Ok(_) => tracing::info!("下載檔案完成: {} (Path: {:?})", &file.name, &file.path),
                    Err(error) => {
                        return Err(error);
                    },
                }
            } else {
                tracing::debug!("本機檔案已存在，跳過: {}", &file.name);
            }
    
            Ok(())
        }
    ).await?;

    tracing::info!("所有資源驗證及下載已完成 OK!");

    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn download_file(url: &str, path: &Path, name: &str, sha1: &str, relative_url: Option<&String>, manifest_urls: Option<&Vec<String>>) -> crate::Result<()> {

    tracing::debug!("開始下載檔案: {} (URL: {})", &name, &url);

    let mut attempt = 0;
    let max_attempts = FETCH_ATTEMPTS + 2;
    // let mut backoff_time = Duration::from_secs(2); // 初始延遲時間

    loop {
        
        attempt += 1;

        let response = REQWEST_CLIENT.request(Method::GET, url);
        let result = response.send().await;

        match result {
            Ok(mut res) => {

                if !res.status().is_success() {

                    tracing::debug!("下載檔案失敗，狀態碼: {}，嘗試重新取得，嘗試次數: {}", res.status(), attempt);

                    if let Some(manifest_urls) = manifest_urls {

                        // tracing::debug!("嘗試使用備用清單 URL 列表取得, URL: {}", url);
                        tracing::debug!("嘗試使用備用清單，共 {} 個 URL", manifest_urls.len());

                        let relative_url = relative_url.ok_or_else(|| {
                            crate::ErrorKind::DownloadFileError("relative_url not None".to_owned())
                        })?;
        
                        let mut status: bool = false;
        
                        for manifest_url in manifest_urls.iter() {
        
                            if status {
                                attempt = 1; // Restart attempt = 1
                                break;
                            }

                            let url = format!("{}{}", manifest_url, relative_url);
                            tracing::debug!("(備用)下載檔案 URL: {}", url);
                            let backup_response = REQWEST_CLIENT.request(Method::GET, &url);
                            let result = backup_response.send().await;

                            match result {
                                Ok(backup_res) => {
                                    if backup_res.status().is_success() {
                                        tracing::debug!("成功使用備用列表 URL 下載檔案: {}", &name);
                                        res = backup_res;
                                        status = true;
                                        break;
                                    }
                                    tracing::error!("使用備用 URL 下載檔案失敗，狀態碼: {}", &res.status());
                                },
                                Err(err) => {
                                    tracing::error!("使用備用 URL 下載檔案失敗: {} (錯誤: {:?})", &name, err);
                                }
                            }
                        }

                        if !status {
                            if attempt > max_attempts {
                                tracing::error!("嘗試使用所有備用清單重新取得下載檔案失敗: {}", &name);
                                return Err(crate::ErrorKind::DownloadFileError(format!("嘗試使用所有備用清單重新取得下載檔案失敗: {}", &name)).as_error());
                            } else {
                                continue;
                            }
                        }
                    } else if attempt < max_attempts {
                        continue;
                    } else {
                        tracing::error!("嘗試重新取得下載檔案失敗: {} (URL: {})", name, url);
                        break;
                        // return Err(crate::ErrorKind::DownloadFileError(format!("嘗試重新取得下載檔案失敗: {} (URL: {})", name, url)).as_error());
                    }
                }

                attempt = 1; // Restart attempt = 1

                let bytes = res.bytes().await?;
                tracing::info!("下載檔案成功，檔案大小: {} bytes", bytes.len());
                util::io::create_dir_all(&path).await?;
                util::io::write_file(path, &bytes).await?;

                // 檢查下载文件的 SHA-1 哈希值是否匹配
                if !sha1_exists(path, sha1)? {

                    util::io::remove_file(path).await?;
                    tracing::debug!("已刪除不匹配的檔案: {:?}", path.to_string_lossy().to_owned());
                    tracing::debug!("檔案雜湊值不符，嘗試重新取得 Name: {} SHA-1: {} URL: {}", name.to_owned(), sha1, url.to_owned());

                    if attempt < max_attempts {
                        continue;
                    } else {
                        tracing::error!("文件 SHA-1 哈希不匹配: {} (SHA-1: {})", name.to_owned(), sha1);
                        return Err(ErrorKind::FileSHA1Error(name.to_owned(), sha1.to_owned(), url.to_owned()).as_error());
                    }
                }

                return Ok(());
            },
            Err(err) => {
                if attempt < max_attempts {
                    continue;
                } else {
                    tracing::error!("下載檔案失敗, URL: {} (錯誤: {:?})", url, err);
                }
            }
        }

        break;
    }

    return Err(crate::ErrorKind::DownloadFileError(format!("下載檔案失敗，已超過最大嘗試次數, URL: {}", url)).as_error())
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
    // if metadata.len() == 0 {
    //     tracing::debug!("本地檔案為空，無法計算 SHA-1");
    //     return Ok(true);
    // }

    let mut hasher = Sha1::new();
    copy(&mut local_file, &mut hasher)?;
    let binding = format!("{:x}", hasher.finalize()).to_owned();
    let local_hash = binding.trim();

    tracing::debug!("本地檔案的雜湊值: {}，預期雜湊值: {}", local_hash, expected_hash);

    Ok(local_hash == expected_hash)
}