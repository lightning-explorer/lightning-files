use core::directory_nav_service::service::*;
mod core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_files_as_dtos,
            format_path_into_dir,
            get_directory_path,
            get_root_path,
            get_parent_directory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
