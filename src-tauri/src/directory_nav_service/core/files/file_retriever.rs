use crate::directory_nav_service::models::get_files_model::GetFilesParamsModel;
use crate::directory_nav_service::util::metadata_inspector::is_hidden;
use crate::FilesDisplayState;

use crate::shared::dtos::file_dto::FileDTO;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use tauri::AppHandle;
use tauri::Emitter;
use tauri::State;

#[tauri::command]
pub async fn get_files_as_dtos(
    directory: String,
    params: GetFilesParamsModel,
    app_handle: AppHandle,
    state_files_display: State<'_, Arc<RwLock<FilesDisplayState>>>,
) -> Result<(), String> {
    // erase the file dtos in the Tauri state:

    if let Ok(mut state) = FilesDisplayState::lock(&state_files_display) {
        state.clear_dtos();
    } else {
        return Err("Failed to clear DTOs in state".to_string());
    }

    let path = Path::new(&directory);
    let entries = fs::read_dir(path).map_err(|_| "Failed to read directory")?;

    // flatten the iterator to remove the 'Err' 'DirEntries' from the loop
    for entry in entries.flatten() {
        let path = entry.path();

        if !params.show_hidden && is_hidden(&path) {
            continue;
        }

        if let Some(dto) = create_dto_from_path(path.clone()).await {
            if let Ok(mut state) = FilesDisplayState::lock(&state_files_display) {
                state.add_dto(dto.clone());
            } else {
                return Err("Failed to add DTO to state".to_string());
            }

            app_handle.emit("file_dto", dto).unwrap_or_default();
        }
    }
    Ok(())
}

// Asynchronous helper function to create a FileDTO
async fn create_dto_from_path(file_path: PathBuf) -> Option<FileDTO> {
    let is_directory = file_path.is_dir();
    let file_name = file_path.file_stem()?.to_string_lossy().to_string();

    Some(FileDTO {
        name: file_name,
        file_path: file_path.to_string_lossy().to_string(),
        metadata: "".to_string(),      // Add metadata logic if needed
        date_modified: "".to_string(), // Add date logic if needed
        score: 0.0,
        is_directory,
    })
}
