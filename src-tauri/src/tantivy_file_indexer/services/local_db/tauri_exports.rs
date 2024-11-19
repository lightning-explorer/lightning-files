use super::tables::crawler_queue::entities::indexed_dir;
use std::sync::Arc;
use tauri::State;

use super::service::LocalDbService;

#[tauri::command]
pub async fn view_crawler_queue(
    service: State<'_, Arc<LocalDbService>>,
) -> Result<Vec<indexed_dir::Model>, String> {
    service
        .crawler_queue_table()
        .view_all()
        .await
        .map_err(|err| format!("Error viewing crawler queue: {}", err))
}
