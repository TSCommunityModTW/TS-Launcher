use serde::ser::SerializeStruct;

pub mod store;
pub mod auth;
pub mod java;
pub mod system;

#[derive(Debug, thiserror::Error)]
pub enum AllayCoreSerializableError {

    #[error("{0}")]
    AllayCore(#[from] allay_core::Error),

    #[error("Store error: {0}")]
    StoreError(String),

    #[error("Tauri invoke error: {0}")]
    TauriInvokeError(#[from] java::JavaInvokeError)
}

impl serde::Serialize for AllayCoreSerializableError {

    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> where S: serde::ser::Serializer {

        match self {
            AllayCoreSerializableError::AllayCore(allay_core_error) => {

                crate::error::display_tracing_error(allay_core_error);

                let mut state = serializer.serialize_struct("AllayCore", 2)?;
                state.serialize_field("field_name", "AllayCoreError")?;
                state.serialize_field("message", &allay_core_error.to_string())?;
                state.end()
            }
            _ => {
                serializer.serialize_str(self.to_string().as_ref())
            }
        }

    }
}

pub type Result<T> = std::result::Result<T, AllayCoreSerializableError>;