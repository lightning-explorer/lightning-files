use std::path::Path;

use super::util::path_ops;

#[tauri::command]
pub fn get_directory_path(file_path: &str) -> String {
    path_ops::get_directory_path(file_path)
        .map(|x| x.to_string_lossy().to_string())
        .unwrap_or(file_path.to_string())
}

#[tauri::command]
pub fn get_root_path(file_path: &str) -> String {
    path_ops::get_root_path(file_path)
        .map(|x| x.to_string_lossy().to_string())
        .unwrap_or(file_path.to_string())
}

#[tauri::command]
pub fn get_parent_directory(file_path: &str) -> String {
    let path = Path::new(file_path);

    let dir_path = if path.is_dir() {
        Some(path)
    } else {
        path.parent()
    };
    if let Some(path) = dir_path {
        path.parent()
            .map(|x| x.to_string_lossy().to_string())
            .unwrap_or(file_path.to_string())
    } else {
        file_path.to_string()
    }
}
