use std::sync::Arc;
use tauri::State;

use super::service::FileCrawlerService;

#[tauri::command]
pub fn add_dirs_to_crawler_queue(
    directories: Vec<&str>,
    service: State<'_, Arc<FileCrawlerService>>,
) {
    service.push_dirs(directories);
}
