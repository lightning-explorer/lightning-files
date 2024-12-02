use std::{
    path::PathBuf,
    sync::Arc,
    time::{Duration, UNIX_EPOCH},
};

use crate::tantivy_file_indexer::{
    dtos::file_dto_input::FileDTOInput,
    shared::indexing_crawler::{
        models::crawler_file::CrawlerFile, traits::crawler_queue_api::CrawlerQueueApi,
    },
};
// refactor
use super::worker_manager::retry_with_backoff;

pub enum CrawlerError {
    ReadDirError(String),
    PushToQueueError(String),
}

/// Where `file` should ideally be a directory. If its not, it will get ignored. Note that this is not a recursive crawl.
/// Returns an `Error` if the found directories failed to get pushed to the crawler queue or there was an error reading the directory.
///
/// Note that the write operation to the queue will retry up to 5 times before finally returning an error.
pub async fn crawl<C>(file: &CrawlerFile, queue: Arc<C>) -> Result<Vec<FileDTOInput>, CrawlerError>
where
    C: CrawlerQueueApi,
{
    if !file.path.is_dir() {
        return Ok(Vec::new());
    }

    let mut dir = tokio::fs::read_dir(&file.path)
        .await
        .map_err(|err| CrawlerError::ReadDirError(err.to_string()))?;

    let mut dtos = Vec::new();
    let mut dir_paths_found: Vec<CrawlerFile> = Vec::new();
    while let Ok(Some(entry)) = dir.next_entry().await {
        let entry_path = entry.path();
        if entry_path.is_dir() {
            dir_paths_found.push(CrawlerFile {
                path: entry_path.clone(),
                priority: file.priority + 1,
                taken: false,
            });
        }
        if let Ok(dto) = create_dto(entry_path).await {
            dtos.push(dto);
        }
    }
    // Push the entries in bulk
    retry_with_backoff(
        || queue.push(&dir_paths_found),
        5,
        Duration::from_millis(1000),
    )
    .await
    .map_err(CrawlerError::PushToQueueError)?;

    Ok(dtos)
}

async fn create_dto(entry: PathBuf) -> Result<FileDTOInput, String> {
    let metadata = entry.metadata().map_err(|x| x.to_string())?;

    let modified_time = metadata.modified().map_err(|x| x.to_string())?;

    let unix_timestamp = modified_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let name = entry
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let dto = FileDTOInput {
        name,
        file_path: entry.to_string_lossy().to_string(),
        metadata: "test metadata".to_string(),
        date_modified: unix_timestamp,
        popularity: 1.0,
    };
    Ok(dto)
}
