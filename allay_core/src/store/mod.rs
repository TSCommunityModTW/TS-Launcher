use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};
use crate::process::children::Children;
use crate::util::app_path;

pub mod launcher_assets;

mod settings;
pub use self::settings::*;

mod profiles;
pub use self::profiles::*;

mod instances;
pub use self::instances::*;

pub struct ValueGetSet<'a, T>(pub &'a mut T);

impl<'a, T> ValueGetSet<'a, T> {
    
    pub fn get(&self) -> &T {
        &self.0
    }

    pub fn set(&mut self, value: T) {
        *self.0 = value;
    }
}

static LAUNCHER_STORE: OnceCell<RwLock<Store>> = OnceCell::const_new();

pub struct Store {
    pub profiles: RwLock<Profiles>,
    pub settings: RwLock<Settings>,
    pub instances: RwLock<Instances>,
    pub children: RwLock<Children>
}

impl Store {

    async fn initialize_store() -> crate::Result<RwLock<Store>> {

        let settings_path = app_path::get_settings_json_file_path();
        let settings = Settings::init(&settings_path).await?;

        let profile_path = app_path::get_profile_json_file_path();
        let profile = Profiles::init(&profile_path).await?;

        let instances_path = app_path::get_instances_json_file_path();
        let mut instances = Instances::new();
        instances.init(&instances_path).await?;

        let children = Children::new();

        Ok(RwLock::new(Self {
            profiles: RwLock::new(profile),
            settings: RwLock::new(settings),
            instances: RwLock::new(instances),
            children: RwLock::new(children)
        }))
    }

    pub async fn get() -> crate::Result<Arc<tokio::sync::RwLockReadGuard<'static, Self>>> {
        Ok(Arc::new(LAUNCHER_STORE.get_or_try_init(Self::initialize_store).await?.read().await))
    }

    pub async fn get_write() -> crate::Result<tokio::sync::RwLockWriteGuard<'static, Self>> {
        Ok(LAUNCHER_STORE.get_or_try_init(Self::initialize_store).await?.write().await)
    }

    pub async fn sync() -> crate::Result<()> {

        let settings_path = app_path::get_settings_json_file_path();
        let profile_path = app_path::get_profile_json_file_path();

        let state = Self::get().await?;

        let sync_settings = async {
            let state = Arc::clone(&state);
            tokio::spawn(async move {
                let settings = state.settings.read().await;
                settings.sync(&settings_path).await?;
                Ok::<_, crate::Error>(())
            }).await?
        };

        let sync_profiles = async {
            let state = Arc::clone(&state);
            tokio::spawn(async move {
                let profiles = state.profiles.read().await;
                profiles.sync(&profile_path).await?;
                Ok::<_, crate::Error>(())
            }).await?
        };

        tokio::try_join!(sync_settings, sync_profiles)?;

        Ok(())
    }
}