use tracing_error::InstrumentError;

use crate::util::{io, java};

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {

    #[error("Fetching error URL: {0}")]
    FetchError(#[from] reqwest::Error),

    #[error("Serialization error (JSON): {0}")]
    JSONError(#[from] serde_json::Error),

    #[error("File SHA-1 hash does not match, {0} sha1: {1} url: {2}")]
    FileSHA1Error(String, String, String),

    #[error("File Download error, expected: {0}")]
    DownloadFileError(String),

    #[error("Files Download error, failure: {0}")]
    DownloadFilesError(usize),

    #[error("I/O (std) error: {0}")]
    StdIOError(#[from] std::io::Error),

    #[error("Create file error, expected: {0}")]
    CreateFileIOError(String),

    #[error("Run future error: {0}")]
    FutureError(#[from] tokio::task::JoinError),

    #[error("Error launching minecraft: {0}")]
    LauncherError(String),

    #[error("I/O error: {0}")]
    IOError(#[from] io::IOError),

    #[error("Error fetching modLoader: {0}")]
    LoaderError(String),

    #[error("Error while parsing version as semver: {0}")]
    SemVerError(#[from] semver::Error),

    #[error("Microsoft auth error: {0}")]
    MicrosoftAuthError(String),

    #[error("Minecraft auth error: {0}")]
    MinecraftAuthError(String),

    #[error("Zip error: {0}")]
    ArchiveZipError(#[from] zip::result::ZipError),

    #[error("{0}")]
    KeyringError(#[from] keyring::Error),

    #[error("Java install error: {0}")]
    JavaUtilError(#[from] java::JavaUtilError),
    
    #[error("Invalid input: {0}")]
    InputError(String),

    #[error("Event error: {0}")]
    EventError(#[from] crate::event::EventError),

    #[error("Unable to read {0} from any source")]
    NoValueFor(String),

    #[error("API interacting error: {0}")]
    APIInteractingError(String)
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