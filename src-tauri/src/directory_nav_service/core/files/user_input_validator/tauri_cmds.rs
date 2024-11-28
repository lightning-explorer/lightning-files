use std::path::Path;

#[tauri::command]
pub fn format_path_into_dir(path: &str) -> Option<String> {
    let os_path = Path::new(path);
    if let Ok(true) = os_path.try_exists() {
        if os_path.is_dir() {
            return Some(path.to_string());
        } else {
            return os_path
                .parent()
                .map(|x| Some(x.to_string_lossy().to_string()))
                .unwrap_or(None);
        }
    }
    None
}
