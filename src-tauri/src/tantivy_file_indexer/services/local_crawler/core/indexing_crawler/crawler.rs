use std::{os::windows::fs::MetadataExt, path::PathBuf, sync::Arc, time::Duration};

use crate::{shared::models::sys_file_model::SystemFileModel, tantivy_file_indexer::{
    converters::date_converter::system_time_to_chrono_datetime,
    shared::{
        async_retry,
        indexing_crawler::{
            models::crawler_file::CrawlerFile, traits::crawler_queue_api::CrawlerQueueApi,
        },
    },
}};
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
) -> Result<Vec<SystemFileModel>, CrawlerError>
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

        match entry.metadata().await {
            Ok(metadata) => match create_sys_file_model(entry_path.clone(), &metadata) {
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
                        "Crawler failed to generate DTO for file: {}. Error: {}",
                        entry_path.to_string_lossy(),
                        err
                    );
                }
            },
            Err(err) => {
                println!(
                    "Crawler failed to get metadata for file: {}. Error: {}",
                    entry_path.to_string_lossy(),
                    err
                );
            }
        }
    }
    // Push the entries in bulk
    async_retry::retry_with_backoff(
        || queue.push(&dir_paths_found),
        5,
        Duration::from_millis(1000),
    )
    .await
    .map_err(CrawlerError::PushToQueueError)?;

    Ok(dtos)
}

fn create_sys_file_model(
    entry: PathBuf,
    entry_meta: &std::fs::Metadata,
) -> Result<SystemFileModel, String> {
    let size = entry_meta.file_size();

    let date_modified =
        system_time_to_chrono_datetime(entry_meta.modified().map_err(|err| err.to_string())?);

    let date_created =
        system_time_to_chrono_datetime(entry_meta.created().map_err(|err| err.to_string())?);

    let name = entry
        .file_name()
        .ok_or("File name was badly formatted")
        .map_err(|err| err.to_string())?
        .to_string_lossy()
        .to_string();

    let dto = SystemFileModel {
        name,
        file_path: entry.to_string_lossy().to_string(),
        metadata: "test metadata".to_string(),
        date_modified,
        date_created,
        size,
        popularity: 1.0,
    };
    Ok(dto)
}
