use std::path::{PathBuf, Path};

use crate::util::java::{self, Arch};

#[tracing::instrument]
pub fn get_java_max_memory_size() -> crate::Result<u64> {
    Ok(java::get_max_memory_size()?)
}

#[tracing::instrument]
pub fn get_free_memory_size() -> crate::Result<u64> {
    Ok(java::get_free_memory_size()?)
}

#[tracing::instrument]
pub async fn install_jar(version: u32) -> crate::Result<PathBuf> {

    //! MacOS 先暫時使用 x86_64
    #[cfg(target_os = "macos")]
    return Ok(java::install(version, Some(Arch::X86_64)).await?);

    #[cfg(target_os = "windows")]
    return Ok(java::install(version, None).await?);

    #[cfg(target_os = "linux")]
    return Ok(java::install(version, None).await?);
}

#[tracing::instrument]
pub async fn test_jar(path: &Path) -> crate::Result<bool> {
    if let Some(is_test) = java::check_java_path(path).await {
        Ok(is_test)
    } else {
        Err(crate::error::ErrorKind::APIInteractingError(format!("Test java jar path error, {}", path.to_path_buf().to_string_lossy())).as_error())
    }
}

// #[tracing::instrument]
// pub async fn install_arch_jar(version: u32, arch: Arch) -> crate::Result<PathBuf> {
//     Ok(java::install(version, Some(arch)).await?)
// }