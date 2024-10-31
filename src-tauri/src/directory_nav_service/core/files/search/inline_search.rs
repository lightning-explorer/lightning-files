use std::sync::{Arc, RwLock};

use crate::shared::dtos::file_dto::FileDTO;
use crate::{directory_nav_service::dtos::inline_query_dto::InlineQueryDTO, FilesDisplayState};
use tauri::State;

#[tauri::command]
pub fn search_files_inline(
    query: InlineQueryDTO,
    files_display: State<'_, Arc<RwLock<FilesDisplayState>>>,
) -> Vec<FileDTO> {
    let state = files_display
        .read()
        .expect("Failed to lock files display state");
    state.query(query)
}
