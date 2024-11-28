use super::super::super::super::app_state::files_display::FilesDisplayState;
use crate::directory_nav_service::models::get_files_model::GetFilesParamsModel;
use crate::directory_nav_service::util::metadata_inspector::is_hidden;

use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;
use tauri::AppHandle;
use tauri::Emitter;
use tauri::State;

use super::helper;

#[tauri::command]
pub fn get_files_as_models(
    directory: String,
    params: GetFilesParamsModel,
    app_handle: AppHandle,
    state_files_display: State<'_, Arc<RwLock<FilesDisplayState>>>,
) -> Result<(), String> {
    // erase the file models in the Tauri state:

    if let Ok(mut state) = FilesDisplayState::lock(&state_files_display) {
        state.clear_files();
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

        if let Some(model) = helper::create_file_model_from_path(path.clone()) {
            if let Ok(mut state) = FilesDisplayState::lock(&state_files_display) {
                state.add_file(model.clone());
            } else {
                return Err("Failed to add DTO to state".to_string());
            }

            app_handle.emit("sys_file_model", model).unwrap_or_default();
        }
    }
    Ok(())
}
