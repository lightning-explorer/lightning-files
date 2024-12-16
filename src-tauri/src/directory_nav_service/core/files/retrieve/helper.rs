use std::{os::windows::fs::MetadataExt, path::PathBuf};

use crate::shared::models::sys_file_model::SystemFileModel;

pub fn create_file_model_from_path(file_path: PathBuf) -> Option<SystemFileModel> {
    match file_path.metadata() {
        Ok(meta) => {
            let is_directory = meta.is_dir();
            let size = meta.file_size();
            let file_name = file_path.file_name()?.to_string_lossy().to_string();

            Some(SystemFileModel {
                name: file_name,
                file_path: file_path.to_string_lossy().to_string(),
                date_modified: chrono::Utc::now(), // Add date logic if needed
                date_created: chrono::Utc::now(),
                metadata: "".to_string(),
                size,
                popularity: 0.0,
                is_directory
            })
        }
        Err(err) => None,
    }
}
