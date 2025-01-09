use tauri::{AppHandle, Manager};
use std::sync::Arc;

use super::services::watcher::service::DirectoryWatcherService;


pub fn manage_state(handle:&AppHandle){
    handle.manage(Arc::new(DirectoryWatcherService::new()));
}