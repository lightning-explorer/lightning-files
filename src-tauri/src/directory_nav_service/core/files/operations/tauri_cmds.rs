use std::path::Path;

use super::{super::super::super::util::path_ops, cmd_prompt, common, file_reader, metadata};

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

/// If the path is already at its root, it will just return the same string. Should also work for files that don't exist
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

/// Returns a Uint8Array
#[tauri::command]
pub fn read_file_bytes(file_path: String, buffer_size:usize)-> Result<Vec<u8>, String>{
    file_reader::read_file_bytes(file_path, buffer_size)
}

/// Returns a String and yields the entire file contents
#[tauri::command]
pub fn read_file(file_path: String)-> Result<String, String>{
    file_reader::read_file(file_path)
}

#[tauri::command]
pub fn read_file_range(file_path: String, start:u64, length:usize)-> Result<String, String>{
    file_reader::read_file_range(file_path, start, length).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn read_file_range_bytes(file_path: String, start:u64, length:usize)-> Result<Vec<u8>, String>{
    file_reader::read_file_range_bytes(file_path, start, length).map_err(|err| err.to_string())
}

/**
 * As opposed to being a directory
 */
#[tauri::command]
pub fn is_path_a_file(file_path: &str) -> bool {
    let path = Path::new(file_path);
    !path.is_dir()
}

#[tauri::command]
pub async fn open_file(file_path: &str) -> Result<(), String> {
    tokio::process::Command::new("cmd")
        .args(["/C", "start", "", file_path])
        .spawn()
        .map_err(|x| x.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn is_directory_accessible(dir_path:&str)->bool{
    metadata::is_directory_accessible(dir_path)
}

// Common commands:
#[tauri::command]
pub fn move_path_into_directory(target_dir: &str, source_path: &str)->Result<(), String>{
    common::move_path_into_directory(Path::new(target_dir), Path::new(source_path)).map_err(|err|err.to_string())
}

#[tauri::command]
pub fn delete_file(file_path:&str)->Result<(), String>{
    common::delete_path(file_path).map_err(|err|err.to_string())
}

#[tauri::command]
pub fn open_in_explorer(path: &str) -> Result<(), String>{
    cmd_prompt::open_in_explorer(path)
}