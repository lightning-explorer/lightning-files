use std::{path::PathBuf, sync::Arc, time::Duration};

use tokio::fs::ReadDir;

use crate::{
    shared::models::sys_file_model::SystemFileModel,
    tantivy_file_indexer::shared::{
        async_retry,
        indexing_crawler::{
            models::crawler_file::CrawlerFile, traits::crawler_queue_api::CrawlerQueueApi,
        },
    },
};

use super::plugins::filterer::{CrawlerFilterer, ShouldIndexResult};

pub enum CrawlerError {
    ReadDirError(String),
    PushToQueueError(String),
}

/// Where `file` should ideally be a directory. If its not, it will get ignored. Note that this is not a recursive crawl.
/// Returns an `Error` if the found directories failed to get pushed to the crawler queue or there was an error reading the directory.
///
/// Note that the write operation to the queue will retry up to 5 times before finally returning an error.
pub async fn crawl<C>(
    file: &CrawlerFile,
    queue: Arc<C>,
    filterer: Option<Arc<CrawlerFilterer>>,
) -> Result<Vec<SystemFileModel>, CrawlerError>
where
    C: CrawlerQueueApi,
{
    if !file.path.is_dir() {
        return Ok(Vec::new());
    }

    let mut dtos = Vec::new();
    let mut dir_paths_found: Vec<CrawlerFile> = Vec::new();

    let mut dir = read_dir(&file.path).await?;

    while let Ok(Some(entry)) = dir.next_entry().await {
        let entry_path = entry.path();
        if let Ok(metadata) = entry_path.metadata() {
            // First, see if the path can be filtered
            if let Some(filterer) = &filterer {
                if let ShouldIndexResult::ShouldNotIndex(_reason) =
                    filterer.should_index(&entry_path).await
                {
                    // Optional log:
                    //println!("Crawler Filterer - found path that shouldn't be indexed: {}. Reason: {}",entry_path.to_string_lossy(),reason);
                    continue;
                }
            }

            match SystemFileModel::try_new_from_meta(entry_path.clone(), &metadata) {
                Ok(dto) => {
                    dtos.push(dto);
                    // If it is a directory, push it to the queue so that it can get processed
                    if metadata.is_dir() {
                        dir_paths_found.push(CrawlerFile {
                            path: entry_path,
                            priority: file.priority + 1,
                            taken: false,
                        });
                    }
                }
                Err(err) => {
                    println!(
                        "Crawler failed to generate model for file: {}. Error: {}",
                        entry_path.to_string_lossy(),
                        err
                    );
                }
            }
        }
    }

    // Push the entries in bulk
    async_retry::retry_with_backoff(
        |_| queue.push(&dir_paths_found),
        5,
        Duration::from_millis(1000),
    )
    .await
    .map_err(CrawlerError::PushToQueueError)?;

    Ok(dtos)
}

async fn read_dir(dir_path: &PathBuf) -> Result<ReadDir, CrawlerError> {
    let dir = tokio::fs::read_dir(&dir_path)
        .await
        .map_err(|err| CrawlerError::ReadDirError(err.to_string()))?;
    Ok(dir)
}
