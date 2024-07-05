use crate::{Java, SelectedServerStart, Settings, Store};

#[tracing::instrument]
pub async fn get() -> crate::Result<Settings> {
    let store = Store::get().await?;
    let settings = store.settings.read().await;
    Ok(settings.clone())
}

#[tracing::instrument]
pub async fn set(settings: Settings) -> crate::Result<()> {
    let store = Store::get().await?;
    *store.settings.write().await = settings;
    Ok(())
}

#[tracing::instrument]
pub async fn get_java(id: &str) -> crate::Result<Java> {

    let settings = self::get().await?;

    if let Some(java) = settings.java.get(id) {
        return Ok(java.clone());
    } else {

        return Ok(Java {
            id: id.to_owned(),
            java17_path: "".to_owned(),
            java16_path: "".to_owned(),
            java8_path: "".to_owned(),
            java_path: "".to_owned(),
            ram_max_size: 4096,
            ram_min_size: 4096,
            java_parameter: "".to_owned(),
            is_built_in_java_vm: true,
            ram_checked: false,
            java_path_checked: false,
            java_parameter_checked: false,
        });

        // return Err(crate::error::ErrorKind::APIInteractingError(format!("Store settings get java hashmap error.")).as_error());
    }
}

#[tracing::instrument]
pub async fn set_java(id: &str, value: Java) -> crate::Result<()> {
    
    let store = Store::get().await?;
    let mut settings = store.settings.write().await;

    if let Some(java) = settings.java.get_mut(id) {
        *java = value;
    } else {
        settings.java.insert(id.to_owned(), value);
        tracing::warn!("Store settings set {} java hashmap . ", id);
    }

    Ok(())
}

#[tracing::instrument]
pub async fn get_selected_server_start() -> crate::Result<(SelectedServerStart)> {
    let settings = self::get().await?;
    return Ok(settings.selected_server_start.clone());
}


#[tracing::instrument]
pub async fn set_selected_server_start(selected_server_start: SelectedServerStart) -> crate::Result<()> {
    let store = Store::get().await?;
    let mut settings = store.settings.write().await;
    settings.selected_server_start=selected_server_start;
    Ok(())
}
