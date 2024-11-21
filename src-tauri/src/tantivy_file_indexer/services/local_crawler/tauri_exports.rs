use std::{path::Path, sync::Arc};
use tauri::State;

use crate::tantivy_file_indexer::dtos::add_to_crawler_queue::AddToCrawlerQueueDTO;

use super::service::FileCrawlerService;

#[tauri::command]
pub async fn add_dirs_to_crawler_queue(
    directories: Vec<AddToCrawlerQueueDTO>,
    service: State<'_, Arc<FileCrawlerService>>,
) -> Result<(), ()> {
    let len = directories.len();
    service
        .push_dirs(
            directories
                .into_iter()
                .map(|entry| (Path::new(&entry.dir_path).to_path_buf(), entry.priority))
                .collect(),
        )
        .await;
    #[cfg(feature="file_crawler_logs")]    
    println!("Added {} directories to the crawler queue", len);
    Ok(())
}
