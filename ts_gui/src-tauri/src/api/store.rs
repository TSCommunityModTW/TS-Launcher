use allay_core::{data::{Java, Profiles, Settings}, profiles, settings, SelectedServerStart, Store};
use tauri::plugin::TauriPlugin;

pub fn init<R: tauri::Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new("store")
        .invoke_handler(tauri::generate_handler![
            save,
            settings_get,
            settings_set,
            get_settings_java,
            set_settings_java,
            profiles_get,
            profiles_set,
            profiles_clear,
            get_settings_selected_server_start,
            set_settings_selected_server_start
        ])
        .build()
}

// invoke("plugin:store|save")
#[tauri::command]
pub async fn save() -> crate::api::Result<()> {
    Store::sync().await?;
    Ok(())
}

// invoke("plugin:store|settings_get")
#[tauri::command]
pub async fn settings_get() -> super::Result<Settings> {
    Ok(settings::get().await?)
}

// invoke("plugin:store|settings_set", settings)
#[tauri::command]
pub async fn settings_set(value: Settings) -> super::Result<()> {
    settings::set(value).await?;
    Ok(())
}

// invoke("plugin:store|get_settings_java", id)
#[tauri::command]
pub async fn get_settings_java(id: &str) -> super::Result<Java> {
    Ok(settings::get_java(id).await?)
}

// invoke("plugin:store|get_settings_java", { id, java })
#[tauri::command]
pub async fn set_settings_java(id: &str, value: Java) -> super::Result<()> {
    settings::set_java(id, value).await?;
    Ok(())
}

// invoke("plugin:store|profiles_get")
#[tauri::command]
pub async fn profiles_get() -> super::Result<Profiles> {
    Ok(profiles::get().await?)
}

// invoke("plugin:store|profiles_set", profiles)
#[tauri::command]
pub async fn profiles_set(value: Profiles) -> super::Result<()> {
    profiles::set(value).await?;
    Ok(())
}

// invoke("plugin:store|profiles_clear")
#[tauri::command]
pub async fn profiles_clear() -> super::Result<()> {
    profiles::clear().await?;
    Store::sync().await?;
    Ok(())
}

// invoke("plugin::store|get_settings_selected_server_start")
#[tauri::command]
pub async fn get_settings_selected_server_start() -> super::Result<SelectedServerStart>{
    Ok(settings::get_selected_server_start().await?)
}


// invoke("plugin::store|set_settings_selected_server_start")
#[tauri::command]
pub async fn set_settings_selected_server_start(value: SelectedServerStart) -> super::Result<()>{
    match settings::set_selected_server_start(value.clone()).await {
        Ok(_) => {
            tracing::debug!("Selected Server Start set: {:?}", value);
            match Store::sync().await {
                Ok(_) => {
                    tracing::debug!("Store synced successfully.");
                    Ok(())
                }
                Err(e) => {
                    tracing::error!("Failed to sync store: {:?}", e);
                    Err(e.into()) // Convert the error to your custom Result type
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to set selected server start: {:?}", e);
            Err(e.into()) // Convert the error to your custom Result type
        }
    }
    
    
    
    
    //settings::set_selected_server_start(value).await?;
    //tracing::debug!("Selected Server Start set:{:?}",value);
    //Store::sync().await?;
    //Ok(())
}
