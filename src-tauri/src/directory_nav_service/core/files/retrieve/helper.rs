use std::path::PathBuf;

use crate::shared::models::sys_file_model::SystemFileModel;

pub fn create_file_model_from_path(file_path: PathBuf) -> Option<SystemFileModel> {
    let is_directory = file_path.is_dir();
    let file_name = file_path.file_name()?.to_string_lossy().to_string();

    Some(SystemFileModel {
        name: file_name,
        file_path: file_path.to_string_lossy().to_string(),
        date_modified: "".to_string(), // Add date logic if needed
        score: 0.0,
        is_directory,
    })
}
