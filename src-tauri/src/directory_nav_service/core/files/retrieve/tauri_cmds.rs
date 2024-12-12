use super::super::super::super::app_state::files_display::FilesDisplayState;
use crate::directory_nav_service::models::get_files_model::GetFilesParamsModel;
use crate::directory_nav_service::util::metadata_inspector::is_hidden;

use std::fs;
use std::path::Path;
use std::sync::Arc;
use tauri::AppHandle;
use tauri::Emitter;
use tauri::State;

use super::helper;

#[tauri::command]
pub async fn get_files_as_models(
    directory: String,
    params: GetFilesParamsModel,
    app_handle: AppHandle,
    state_files_display: State<'_, Arc<FilesDisplayState>>,
) -> Result<(), String> {
    // Erase the file models in the Tauri state:
    state_files_display.clear_files().await;
    let path = Path::new(&directory);
    let entries = fs::read_dir(path).map_err(|_| "Failed to read directory")?;

    // flatten the iterator to remove the 'Err' 'DirEntries' from the loop
    let mut files_to_add = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();

        if !params.show_hidden && is_hidden(&path) {
            continue;
        }

        if let Some(model) = helper::create_file_model_from_path(path.clone()) {
            files_to_add.push(model.clone());
            app_handle.emit("sys_file_model", model).unwrap_or_default();
        }
    }
    state_files_display.add_files(&mut files_to_add).await;

    Ok(())
}
