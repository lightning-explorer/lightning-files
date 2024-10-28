use core::filesystem::file_retriever::get_files_as_dtos;
use core::filesystem::user_input_validator::format_path_into_dir;
mod core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_files_as_dtos, format_path_into_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
