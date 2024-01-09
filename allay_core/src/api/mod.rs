// * API for interacting with Allay-core

pub mod settings;
pub mod java;
pub mod auth;
pub mod profiles;
pub mod launcher;
pub mod process;

pub mod data {

    pub use crate:: {
        util::oauth::microsoft::DeviceAuth,
        store:: {
            launcher_assets::LauncherAssets,
            Settings,
            Profiles,
            Java
        }
    };
}

pub mod prelude {

    pub use crate::api:: {
        process,
        launcher,
        settings,
        java
    };
    
}