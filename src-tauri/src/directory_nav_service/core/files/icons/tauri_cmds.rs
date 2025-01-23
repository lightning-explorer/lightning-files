use getfileicon::api::PngCache;
use std::sync::Arc;
use tauri::State;
use tokio::sync::RwLock;

/// Get the icon of a file as a base64 encoded string
#[tauri::command]
pub async fn get_file_icon(
    path: &str,
    width: u32,
    height: u32,
    cache: State<'_, Arc<RwLock<PngCache>>>,
) -> Result<String, String> {
    let mut cache = cache.write().await;
    match cache.get(path, width, height) {
        Some(image) => image.as_base64_png().map_err(|e| e.to_string()),
        None => Err(format!("File icon not found for path: {}", path)),
    }
}
