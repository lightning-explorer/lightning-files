use std::sync::Arc;

use tauri::State;

use crate::tantivy_file_indexer::services::local_db::service::LocalDbService;

#[tauri::command]
pub async fn get_num_stored_files(service: State<'_, Arc<LocalDbService>>) -> Result<u64, String> {
    match service.files_table_connection().count_files().await {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    }
}
