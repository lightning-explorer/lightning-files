use std::fs;
use std::path::Path;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Emitter;
use super::dtos::file_dto::FileDTO;

#[tauri::command]
pub async fn get_files_as_dtos(directory: String, app_handle: AppHandle) -> Result<(), String> {
    let path = Path::new(&directory);

    // Read directory asynchronously
    let mut entries = fs::read_dir(path).map_err(|_| "Failed to read directory")?;

    while let Some(entry) = entries.next() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(dto) = create_dto_from_path(path.clone()).await {
                app_handle.emit("file_dto", dto).unwrap_or_default();
            }
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
