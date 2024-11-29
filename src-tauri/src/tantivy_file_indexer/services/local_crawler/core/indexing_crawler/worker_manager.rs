use std::{
    collections::HashSet,
    path::PathBuf,
    sync::Arc,
    time::{Duration, UNIX_EPOCH},
};

use tantivy::{schema::Schema, IndexWriter};
use tokio::{
    sync::{Mutex, Notify},
    task::{JoinHandle, JoinSet},
};

use crate::tantivy_file_indexer::{
    dtos::file_dto_input::FileDTOInput,
    services::local_crawler::{
        models::crawler_file::{self, CrawlerFile},
        traits::{crawler_queue_api::CrawlerQueueApi, files_collection_api::FilesCollectionApi},
    },
};

use super::indexer;

pub type TantivyInput = (Arc<Mutex<IndexWriter>>, Schema);

/*
Workflow:
The manager pulls a batch of directories from the crawler queue, then, that many threads will get spawned to crawl and make DTOs out of them.
then they all get indexed
*/

pub async fn manager_task<C, F>(
    crawler_queue: Arc<C>,
    files_collection: Arc<F>,
    tantivy: TantivyInput,
    notify: Arc<Notify>,
    max_concurrent_tasks: usize,
) where
    C: CrawlerQueueApi,
    F: FilesCollectionApi,
{
    let (writer, schema) = tantivy;
    let mut tasks: JoinSet<()> = JoinSet::new();
    loop {
        match crawler_queue
            .fetch((max_concurrent_tasks - tasks.len()) as u64)
            .await
        {
            Ok(paths) => {
                if !paths.is_empty() {
                    // Dispatch the batch of tasks
                    for file in paths {
                        let queue_clone = Arc::clone(&crawler_queue);
                        let files_collection_clone = Arc::clone(&files_collection);
                        let writer_clone = Arc::clone(&writer);
                        let schema_clone = schema.clone();

                        tasks.spawn(async move {
                            if file.path.is_dir() {
                                let mut dtos = Vec::new();
                                let mut dir_paths_found: Vec<CrawlerFile> = Vec::new();

                                if let Ok(mut dir) = tokio::fs::read_dir(&file.path).await {
                                    while let Ok(Some(entry)) = dir.next_entry().await {
                                        let entry_path = entry.path();
                                        if entry_path.is_dir() {
                                            dir_paths_found.push(CrawlerFile {
                                                path: entry_path.clone(),
                                                priority: file.priority + 1,
                                            });
                                        }
                                        if let Ok(dto) = create_dto(entry_path).await {
                                            dtos.push(dto);
                                        }
                                    }
                                }
                                // Add the directories that were found to the queue
                                // TODO: HANDLE THE ERROR
                                queue_clone.push(&dir_paths_found).await;
                                // Perform the indexing operations
                                // TODO: HANDLE THE ERROR
                                indexer::index_files(
                                    dtos,
                                    (writer_clone, schema_clone),
                                    file.path.clone(),
                                    files_collection_clone,
                                )
                                .await;
                            }
                            // If all goes well, then the directory can be removed from the crawler queue
                            queue_clone.delete(&[file.clone()]).await;
                        });
                    }
                } else {
                    // If queue is empty, wait for notification
                    println!("No tasks available. Waiting for notification...");
                    notify.notified().await;
                }
            }
            Err(err) => {
                eprintln!(
                    "File crawler task manager encountered an error: {}. Retrying in 1 second.",
                    err
                );
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }
        }
    }
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
