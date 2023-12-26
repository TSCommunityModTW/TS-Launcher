pub mod auth;
pub mod player;

#[derive(Debug, thiserror::Error)]
pub enum AllayCoreSerializableError {
    #[error("{0}")]
    AllayCore(#[from] allay_core::Error),
}

impl serde::Serialize for AllayCoreSerializableError {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type Result<T> = std::result::Result<T, AllayCoreSerializableError>;