use tracing_error::InstrumentError;

use crate::util::{io, java};

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {

    #[error("抓取錯誤的 URL: {0}")]
    FetchError(#[from] reqwest::Error),

    #[error("序列化錯誤 (JSON): {0}")]
    JSONError(#[from] serde_json::Error),

    #[error("文件 SHA-1 哈希不匹配, {0} sha1: {1} url: {2}")]
    FileSHA1Error(String, String, String),

    #[error("文件下載錯誤，說明: {0}")]
    DownloadFileError(String),

    #[error("多個文件下載錯誤，失敗: {0}")]
    DownloadFilesError(usize),

    #[error("I/O (std) 錯誤: {0}")]
    StdIOError(#[from] std::io::Error),

    #[error("創建文件錯誤，預期: {0}")]
    CreateFileIOError(String),

    #[error("執行 future 錯誤: {0}")]
    FutureError(#[from] tokio::task::JoinError),

    #[error("啟動 Minecraft 錯誤: {0}")]
    LauncherError(String),

    #[error("I/O 錯誤: {0}")]
    IOError(#[from] io::IOError),

    #[error("抓取 modLoader 錯誤: {0}")]
    LoaderError(String),

    #[error("解析版本為 semver 時發生錯誤: {0}")]
    SemVerError(#[from] semver::Error),

    #[error("Microsoft 認證錯誤: {0}")]
    MicrosoftAuthError(String),

    #[error("Minecraft 認證錯誤: {0}")]
    MinecraftAuthError(String),

    #[error("壓縮檔錯誤: {0}")]
    ArchiveZipError(#[from] zip::result::ZipError),

    #[error("{0}")]
    KeyringError(#[from] keyring::Error),

    #[error("Java 安裝錯誤: {0}")]
    JavaUtilError(#[from] java::JavaUtilError),
    
    #[error("無效輸入: {0}")]
    InputError(String),

    #[error("事件錯誤: {0}")]
    EventError(#[from] crate::event::EventError),

    #[error("無法從任何來源讀取 {0}")]
    NoValueFor(String),

    #[error("API 交互錯誤: {0}")]
    APIInteractingError(String),

    #[error("啟動器資源錯誤: {0}")]
    LauncherAssetsError(String),
}

#[derive(Debug)]
pub struct Error {
    source: tracing_error::TracedError<ErrorKind>,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.source()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.source)
    }
}

impl<E: Into<ErrorKind>> From<E> for Error {
    fn from(source: E) -> Self {
        Self {
            source: Into::<ErrorKind>::into(source).in_current_span(),
        }
    }
}

impl ErrorKind {
    pub fn as_error(self) -> Error {
        self.into()
    }
}

pub type Result<T> = core::result::Result<T, Error>;