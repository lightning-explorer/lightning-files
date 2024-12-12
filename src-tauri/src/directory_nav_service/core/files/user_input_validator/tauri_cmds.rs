use std::path::Path;

#[tauri::command]
pub fn format_path_into_dir(path: &str) -> Option<String> {
    let os_path = Path::new(path);

    if os_path.file_name().is_some() && os_path.extension().is_some() {
        return os_path
            .parent()
            .map(|x| Some(x.to_string_lossy().to_string()))
            .unwrap_or(None);
    }
    Some(path.to_string())
}

// directory1/directory2/some_file.txt
// directory1/directory2
