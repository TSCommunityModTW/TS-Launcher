use std::{path::{PathBuf, Path}, process::Command};
use serde::Deserialize;

use crate::{
    util:: {
        fetch:: {
            fetch,
            request_json
        },
        app_path,
        io
    },
    emit:: {
        init_loading,
        emit_loading
    },
    LoadingBarType
};

#[cfg(target_os = "windows")]
#[allow(dead_code)]
pub const JAVA_BIN: &str = "javaw.exe";

#[cfg(not(target_os = "windows"))]
#[allow(dead_code)]
pub const JAVA_BIN: &str = "java";

#[derive(Debug)]
pub enum Arch {
    X86_64,
    Aarch64
}

impl Arch {
    pub fn to_string(&self) -> String {
        match self {
            Arch::X86_64 => "x86_64".to_owned(),
            Arch::Aarch64 => "aarch64".to_owned(),
        }
    }
}

pub async fn install(java_version: u32, arch: Option<Arch>) -> crate::Result<PathBuf> {

    let loading_bar = init_loading(
        LoadingBarType::JavaDownload {
            version: java_version
        },
        100.0,
        &format!("Downloading java {} version", java_version),
    )
    .await?;

    #[derive(Deserialize, Debug, Clone)]
    struct Package {
        pub download_url: String,
        pub name: PathBuf,
    }

    let arch = if let Some(arch) = arch {
        arch.to_string()
    } else {
        std::env::consts::ARCH.to_string()
    };

    let url = format!(
        "https://api.azul.com/metadata/v1/zulu/packages?arch={}&java_version={}&os={}&archive_type=zip&javafx_bundled=false&java_package_type=jre&page_size=1",
        arch, java_version, std::env::consts::OS
    );

    emit_loading(&loading_bar, 5.0, Some(&format!("Fetching java {} version", java_version))).await?;
    let packages = request_json::<Vec<Package>>(&url).await?;

    emit_loading(&loading_bar, 10.0, Some(&format!("Downloading java {} version", java_version))).await?;
    if let Some(download) = packages.first() {

        let java_dir_path = app_path::get_runtime_dir_path().join(java_version.to_string());
        let download_url = &download.download_url;

        let file_bytes = fetch(download_url, None, Some((&loading_bar, 65.0))).await?;

        let mut archive = zip::ZipArchive::new(std::io::Cursor::new(file_bytes))
            .map_err(|_| {
                crate::Error::from(crate::ErrorKind::InputError(
                    "無法讀取 java zip".to_string(),
                ))
            })?;

        // 刪除舊安裝的 java 文件
        if java_dir_path.exists() {
            io::remove_dir_all(&java_dir_path).await?;
        }

        io::create_dir_all(&java_dir_path).await?;

        emit_loading(&loading_bar, 10.0, Some(&format!("Extracting java {}", java_version))).await?;
        archive.extract(&java_dir_path)
            .map_err(|_| {
                crate::Error::from(crate::ErrorKind::InputError(
                    "無法提取 java zip".to_string(),
                ))
            })?;

        emit_loading(&loading_bar, 10.0, Some(&format!("Done extracting java {}", java_version))).await?;
        let mut base_path = java_dir_path.join(
                download
                .name
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        );
            
        #[cfg(target_os = "macos")]
        {
            base_path = base_path
                .join(format!("zulu-{}.jre", java_version))
                .join("Contents")
                .join("Home")
                .join("bin")
                .join("java")
        }
            
        #[cfg(not(target_os = "macos"))]
        {
            base_path = base_path.join("bin").join(JAVA_BIN)
        }

        return Ok(base_path);
    }

    Err(JavaUtilError::Install(format!(
        "未找到 Java 版本 {}, 作業系統 {}, 架構 {} 的 Java 版本",
        java_version, std::env::consts::OS, std::env::consts::ARCH)
    ).into())
}

pub async fn check_java_path(path: &Path) -> Option<bool> {

    // 嘗試規範化潛在的 java 檔案路徑
    // 如果失敗，則路徑不存在，回傳 None（這裡沒有 Java）
    let Ok(path) = io::canonicalize(path) else {
        return Some(false);
    };

    // 檢查此檔案路徑中是否存在 Java
    // 如果 JAVA_BIN 尚不存在，則將其新增至路徑末尾
    let java = if path.file_name()?.to_str()? != JAVA_BIN {
        path.join(JAVA_BIN)
    } else {
        path
    };

    if !java.exists() {
        return Some(false);
    };

    let bytes = include_bytes!("../../library/JavaInfo.class");
    let tempdir: PathBuf = tempfile::tempdir().ok()?.into_path();

    if !tempdir.exists() {
        return None;
    }

    let file_path = tempdir.join("JavaInfo.class");
    io::create_dir_all(&file_path).await.ok()?;
    io::write_file(&file_path, bytes).await.ok()?;

    let output = Command::new(&java)
        .arg("-cp")
        .arg(file_path.parent().unwrap())
        .arg("JavaInfo")
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut java_version = None;
    let mut java_arch = None;

    for line in stdout.lines() {
        let mut parts = line.split("=");
        let key = parts.next().unwrap_or_default();
        let value = parts.next().unwrap_or_default();

        if key == "os.arch" {
            java_arch = Some(value);
        } else if key == "java.version" {
            java_version = Some(value);
        }
    }

    if let Some(arch) = java_arch {
        if let Some(version) = java_version {

            // let path = java.to_string_lossy().to_string();
            
            // TODO
            return Some(true);

        }
    }

    Some(false)
}

pub fn get_max_memory_size() -> crate::Result<u64> {
    // System::new_all().total_memory() / 1024 / 1024
    Ok(sys_info::mem_info().map_err(|_| {
        crate::Error::from(crate::ErrorKind::LauncherError(
            "Unable to get computer memory".to_string(),
        ))
    })?.total / 1024)
}

pub fn get_free_memory_size() -> crate::Result<u64> {
    // System::new_all().used_memory() / 1024 / 1024
    Ok(sys_info::mem_info().map_err(|_| {
        crate::Error::from(crate::ErrorKind::LauncherError(
            "Unable to get computer memory".to_string(),
        ))
    })?.avail / 1024)
}

#[derive(Debug, thiserror::Error)]
pub enum JavaUtilError {
    
    #[error("Java install error, {0}")]
    Install(String),

    // #[error("Check java path error, {0}")]
    // CheckJavaPathError(String)
}

// async fn extract_gz(path: &Path) -> crate::Result<()> {

//     if let Some(parent) = path.parent() {
//         create_dir_all(parent).await?;
//     }

//     let tar_gz = File::open(path)?;
//     let tar = GzDecoder::new(tar_gz);
//     let mut archive = Archive::new(tar);
//     archive.unpack(path.parent().unwrap())?;

//     Ok(())
// }

// async fn extract_zip(path: &Path) -> crate::Result<()> {

//     if let Some(parent) = path.parent() {
//         create_dir_all(parent).await?;
//     }

//     let zip_file = File::open(&path)?;
//     let mut archive = zip::ZipArchive::new(zip_file)?;
//     archive.extract(&path.parent().unwrap())?;

//     Ok(())
// }