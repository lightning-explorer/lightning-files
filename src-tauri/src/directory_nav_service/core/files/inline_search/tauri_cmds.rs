use super::super::super::super::app_state::files_display::FilesDisplayState;
use crate::directory_nav_service::dtos::inline_query_dto::InlineQueryDTO;
use crate::shared::models::sys_file_model::SystemFileModel;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn search_files_inline(
    query: InlineQueryDTO,
    files_display: State<'_, Arc<FilesDisplayState>>,
) -> Result<Vec<SystemFileModel>, ()> {
    Ok(files_display.query(query).await)
}
