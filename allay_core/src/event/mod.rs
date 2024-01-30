use std::{collections::HashMap, sync::Arc};

use serde::{Serialize, Deserialize};
use tokio::sync::{OnceCell, RwLock};
use ts_rs::TS;
use uuid::Uuid;

pub mod emit;

static EVENT_STATE: OnceCell<Arc<EventState>> = OnceCell::const_new();

pub struct EventState {

    #[cfg(feature = "tauri")]
    pub app: tauri::AppHandle,
    pub loading_bars: RwLock<HashMap<Uuid, LoadingBar>>,
}

impl EventState {
    
    #[cfg(feature = "tauri")]
    pub async fn initialize(app: tauri::AppHandle) -> crate::Result<Arc<Self>> {
        EVENT_STATE.get_or_try_init(|| async {
            Ok(Arc::new(Self { app, loading_bars: RwLock::new(HashMap::new()) }))
        }).await.map(Arc::clone)
    }

    #[cfg(not(feature = "tauri"))]
    pub async fn initialize() -> crate::Result<Arc<Self>> {
        EVENT_STATE
            .get_or_try_init(|| async {
                Ok(Arc::new(Self {
                    loading_bars: RwLock::new(HashMap::new()),
                }))
            })
            .await
            .map(Arc::clone)
    }

    #[cfg(feature = "tauri")]
    pub async fn get() -> crate::Result<Arc<Self>> {
        Ok(EVENT_STATE.get().ok_or(EventError::NotInitialized)?.clone())
    }

    #[cfg(not(feature = "tauri"))]
    pub async fn get() -> crate::Result<Arc<Self>> {
        Self::initialize().await
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct LoadingBar {
    pub loading_bar_uuid: Uuid,
    pub message: String,
    pub total: f64,
    pub bar_type: LoadingBarType,
    pub current: f64,
    #[serde(skip)]
    pub last_sent: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq, TS)]
#[ts(rename = "IEventLoadingBarType")]
#[ts(export, export_to = "../ts_gui/src/interfaces/IEventLoadingBarType.ts")]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum LoadingBarType {
    JavaDownload {
        version: u32,
    },
    ProcessChildren {
        id: String,
        server_id: String,
    }
}


#[derive(Serialize, Clone, TS)]
#[ts(rename = "IEventLoadingPayload")]
#[ts(export, export_to = "../ts_gui/src/interfaces/IEventLoadingPayload.ts")]
pub struct LoadingPayload {
    pub event: LoadingBarType,
    #[ts(type = "string")]
    pub loader_uuid: Uuid,
    pub fraction: f64,
    pub message: String
}

#[derive(Serialize, Debug, Clone)]
pub struct LoadingBarId(Uuid);

impl Drop for LoadingBarId {
    fn drop(&mut self) {

        let loader_uuid = self.0;

        tokio::spawn(async move {

            if let Ok(event_state) = EventState::get().await {

                let mut bars = event_state.loading_bars.write().await;

                #[cfg(feature = "tauri")]
                if let Some(bar) = bars.remove(&loader_uuid) {

                    let loader_uuid = bar.loading_bar_uuid;
                    // let event = bar.bar_type.clone();
                    let fraction = bar.current / bar.total;

                    // use tauri::Manager;

                    // let _ = event_state.app.emit_all(
                    //     "loading",
                    //     LoadingPayload {
                    //         fraction: Some(100.0),
                    //         message: "Completed".to_string(),
                    //         event,
                    //         loader_uuid
                    //     },
                    // );

                    tracing::trace!(
                        "Exited at {fraction} for loading bar: {:?}",
                        loader_uuid
                    );
                }

                #[cfg(not(feature = "tauri"))]
                bars.remove(&loader_uuid);
            }
        });
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EventError {
    #[error("Event state was not properly initialized")]
    NotInitialized,

    #[error("Non-existent loading bar of key: {0}")]
    NoLoadingBar(Uuid),

    #[cfg(feature = "tauri")]
    #[error("Tauri error: {0}")]
    TauriError(#[from] tauri::Error),
}