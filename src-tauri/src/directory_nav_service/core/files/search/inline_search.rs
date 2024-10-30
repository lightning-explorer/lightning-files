use std::sync::{Arc, RwLock};

use tauri::State;

use crate::{directory_nav_service::dtos::{file_dto::FileDTO, inline_query_dto::InlineQueryDTO}, FilesDisplayState};

#[tauri::command]
pub fn search_files_inline(query: InlineQueryDTO, files_display: State<'_, Arc<RwLock<FilesDisplayState>>>) -> Vec<FileDTO> {
    let state = files_display.read().expect("Failed to lock files display state");
    state.query(query)
}