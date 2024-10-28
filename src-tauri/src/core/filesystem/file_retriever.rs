use std::fs;
use std::io::Error;
use std::path::Path;
use std::path::PathBuf;

use super::dtos::file_dto::FileDTO;
use super::metadata_inspector::is_hidden;

#[tauri::command]
pub fn get_files_as_dtos(directory: &str) -> Vec<FileDTO> {
    Path::new(directory)
        .read_dir()
        .into_iter()
        .flat_map(|dir| dir.filter_map(Result::ok))
        .filter(|entry| {
            !is_hidden(entry.path().as_path())
        })
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_file() {
                dto_from_path(&path, false).ok()
            } else if path.is_dir() {
                dto_from_path(&path, true).ok()
            } else {
                None
            }
        })
        .collect()
}

fn dto_from_path(file_path: &PathBuf, is_directory: bool) -> Result<FileDTO, &'static str> {
    Path::new(file_path)
        .file_stem()
        .map(|file_name| FileDTO {
            name: file_name.to_string_lossy().to_string(),
            file_path: file_path.to_string_lossy().to_string(),
            metadata: "".to_string(),
            date_modified: "".to_string(),
            score: 0.0,
            is_directory,
        })
        .ok_or("Path is not valid")
}
