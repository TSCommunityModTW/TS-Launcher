use crate::{store::launcher_assets::LauncherAssets, launcher_assets::{Server, ServerChildren}};

pub async fn initialize_assets() -> crate::Result<()> {
    LauncherAssets::initialize().await?;
    Ok(())
}

#[tracing::instrument]
pub fn get() -> crate::Result<LauncherAssets> {
    Ok(LauncherAssets::get()?)
}

#[tracing::instrument]
pub fn get_servers() -> crate::Result<Vec<Server>> {
    Ok(LauncherAssets::get_servers()?)
}

#[tracing::instrument]
pub fn get_server(id: &str) -> crate::Result<Server> {
    if let Some(server) = self::get_servers()?.iter().find(|server| server.id == id) {
        return Ok(server.clone());
    } else {
        Err(crate::ErrorKind::APIInteractingError("啟動器資產會找到對應的 Server ID".to_owned()).as_error())
    }
}

#[tracing::instrument]
pub fn get_children_server(main_server_id: &str, id: &str) -> crate::Result<ServerChildren> {
    if let Some(children_server) = self::get_server(main_server_id)?.children.iter().find(|children| children.id == id) {
        return Ok(children_server.clone());
    } else {
        Err(crate::ErrorKind::APIInteractingError("啟動器資產會找到對應的 Children Server ID".to_owned()).as_error())
    }
}