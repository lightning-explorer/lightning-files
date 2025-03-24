use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::RwLock;

use super::services::watcher::service::DirectoryWatcherService;

pub fn manage_state(handle: &AppHandle) {
    let png_cache = getfileicon::api::PngCache::new(32);
    handle.manage(png_cache);
    handle.manage(Arc::new(DirectoryWatcherService::new()));
}
