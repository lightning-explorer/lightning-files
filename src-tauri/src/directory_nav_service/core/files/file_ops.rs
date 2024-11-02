use std::path::Path;
use crate::directory_nav_service::util::metadata_inspector::is_hidden;

use super::super::super::util::path_ops;

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

/**
 * As opposed to being a directory
 */
#[tauri::command]
pub fn is_path_a_file(file_path:&str)->bool{
    let path = Path::new(file_path);
    !path.is_dir()
}

#[tauri::command]
pub async fn open_file(file_path:&str)-> Result<(),String>{
    tokio::process::Command::new("cmd")
    .args(["/C","start","",file_path])
    .spawn()
    .map_err(|x| x.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn is_file_hidden(file_path:&str)->bool{
    is_hidden(Path::new(file_path))
}