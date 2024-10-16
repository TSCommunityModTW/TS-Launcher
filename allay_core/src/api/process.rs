use uuid::Uuid;

use crate::process::Process;

#[tracing::instrument(skip_all)]
pub async fn process_minecraft_run(server_id: &str, children_server_id: &str) -> crate::Result<()> {    
    Process::run(server_id, children_server_id, Uuid::new_v4()).await?;
    Ok(())
}