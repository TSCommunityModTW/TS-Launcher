// * API for interacting with Allay-core

pub mod settings;
pub mod java;
pub mod auth;
pub mod profiles;

pub mod data {
    pub use crate::util::oauth::microsoft:: {
        DeviceAuth
    };
}

pub mod prelude {

    pub use crate::{
        settings,
        java,
    };
    
}