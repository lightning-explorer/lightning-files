use std::path::PathBuf;

use crate::directory_nav_service::models::sys_file_model::SystemFileModel;


pub fn create_file_model_from_path(file_path: PathBuf) -> Option<SystemFileModel> {
    let is_directory = file_path.is_dir();
    let file_name = file_path.file_stem()?.to_string_lossy().to_string();

    Some(SystemFileModel {
        name: file_name,
        file_path: file_path.to_string_lossy().to_string(),
        date_modified: "".to_string(), // Add date logic if needed
        is_directory,
    })
}
