use crate::{Profiles, Store};

#[tracing::instrument]
pub async fn get() -> crate::Result<Profiles> {
    let store = Store::get().await?;
    let profiles = store.profiles.read().await;
    Ok(profiles.clone())
}

#[tracing::instrument]
pub async fn set(profiles: Profiles) -> crate::Result<()> {
    let store = Store::get().await?;
    *store.profiles.write().await = profiles;
    Ok(())
}