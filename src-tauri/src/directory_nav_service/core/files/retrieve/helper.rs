use std::path::PathBuf;

use crate::shared::dtos::file_dto::FileDTO;

// Asynchronous helper function to create a FileDTO
pub async fn create_dto_from_path(file_path: PathBuf) -> Option<FileDTO> {
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
