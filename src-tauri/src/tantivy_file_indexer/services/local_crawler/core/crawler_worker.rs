
use futures::{stream, StreamExt};
use std::time::Instant;
use std::{path::PathBuf, sync::Arc, time::UNIX_EPOCH};
use tokio::time::{self, Duration};
use tokio::{
    sync::Semaphore,
    task::JoinSet,
};

use crate::tantivy_file_indexer::services::local_crawler::analyzer::service::FileCrawlerAnalyzerService;
use crate::tantivy_file_indexer::shared::local_db_and_search_index::traits::file_sender_receiver::FileIndexerSender;
use crate::tantivy_file_indexer::{
    dtos::file_dto_input::FileDTOInput,
    services::search_index::models::index_worker::file_input::FileInputModel, util::file_id_helper,
};

use super::crawler_queue::CrawlerQueue;

pub async fn spawn_worker<T>(
    sender: T,
    max_concurrent_tasks: usize,
    queue: Arc<CrawlerQueue>,
) where T: FileIndexerSender {
    spawn_worker_internal(sender, max_concurrent_tasks, queue, None).await;
}

pub async fn spawn_worker_with_analyzer<T>(
    sender: T,
    max_concurrent_tasks: usize,
    queue: Arc<CrawlerQueue>,
    analyzer: Arc<FileCrawlerAnalyzerService>,
) where T: FileIndexerSender {
    spawn_worker_internal(sender, max_concurrent_tasks, queue, Some(analyzer)).await;
}

pub async fn spawn_worker_internal<T>(
    sender: T,
    max_concurrent_tasks: usize,
    queue: Arc<CrawlerQueue>,
    analyzer: Option<Arc<FileCrawlerAnalyzerService>>,
) where T: FileIndexerSender {
    let semaphore = Arc::new(Semaphore::new(max_concurrent_tasks));
    let mut tasks = JoinSet::new();
    let worker_queue = Arc::clone(&queue);

    // for logging
    let mut subworker_id: u32 = 0;

    loop {
        if let Some(ref analyzer) = analyzer {
            analyzer.record_timestamp().await;
        }
        if let Some((path, priority)) = queue.pop().await {
            let _permit = Arc::clone(&semaphore).acquire_owned().await.expect("Failed to acquire semaphore");

            let sender = sender.clone();
            let crawler_queue_clone = Arc::clone(&worker_queue);
            let analyzer_clone = analyzer.clone();

            subworker_id += 1;

            tasks.spawn(async move {

                #[cfg(feature = "file_crawler_logs")]
                println!(
                    "File crawler subworker has been spawned to process entries in directory. ID: {}",
                    subworker_id
                );
            

                let mut input_dtos_tasks = Vec::new();
                let mut dir_paths_priority: Vec<(PathBuf,u32)> = Vec::new();
                let mut files_processed:usize = 0;

                if path.is_dir() {
                    if let Ok(mut dir) = tokio::fs::read_dir(&path).await {
                        // Iterate over each entry in the directory
                        while let Ok(Some(entry)) = dir.next_entry().await {

                            let entry_path = entry.path();
                            if entry_path.is_dir() {
                                // This Vec is specific to this thread, so no atomic operations right here
                                dir_paths_priority.push((entry_path.clone(), priority + 1));
                            }
                            // Metadata is fetched here
                            let metadata_fetch_task= tokio::spawn(async move{
                                create_dto(entry_path).await
                            });
                            input_dtos_tasks.push(metadata_fetch_task);

                            files_processed += 1;
                        }
                        // End of the while loop. Put some heavier operations here:
                        // This is a database call here:
                        crawler_queue_clone.push_many(&dir_paths_priority).await;

                        if let Some(ref analyzer) = analyzer_clone {
                            // Adding to an atomic variable:
                            analyzer.add_to_files_processed(files_processed);
                        }
                    }
                }

                // Await the metadata fetch tasks
                let input_dtos: Vec<FileDTOInput> = futures::future::join_all(input_dtos_tasks)
                .await
                .into_iter()
                .flatten()
                .flatten()
                .collect();

                #[cfg(feature = "speed_profile")]
                let time = Instant::now();

                let model = create_model(path, input_dtos).await;

                #[cfg(feature = "speed_profile")]
                println!(
                    "Crawler creating FileInputModel for batch of files/directories took: {:?}",
                    time.elapsed()
                );

                #[cfg(feature = "speed_profile")]
                let time = Instant::now();

                if let Err(err) = sender.send(model).await {
                    eprintln!("Error sending FileInputModel to indexer: {}", err);
                }
                #[cfg(feature = "speed_profile")]
                println!(
                    "Crawler sending files to indexer took: {:?}",
                    time.elapsed()
                );

                #[cfg(feature = "file_crawler_logs")]
                println!(
                    "File crawler subworker has finished. ID: {}",
                    subworker_id
                );
                
            });
        } else {
            #[cfg(feature = "file_crawler_logs")]
            println!("File crawler has nothing to do. Sleeping");
            time::sleep(Duration::from_millis(100)).await;
        }

        while let Some(result) = tasks.join_next().await {
            if let Err(err) = result {
                eprintln!("Task error: {:?}", err);
            }
        }
    }
}

async fn create_model(directory_from: PathBuf, dtos: Vec<FileDTOInput>) -> FileInputModel {
    FileInputModel {
        dtos,
        directory_from,
    }
}

async fn create_dtos_batch(entries: Vec<PathBuf>) -> Vec<Result<FileDTOInput, String>> {
    stream::iter(entries)
        .map(|entry| create_dto(entry))
        .buffer_unordered(16)
        .collect()
        .await
}

// This function takes around 60ms to complete so look at this
// After some testing and removing 'metadata', the function still takes about the same amount of time
async fn create_dto(entry: PathBuf) -> Result<FileDTOInput, String> {
    let metadata = entry.metadata().map_err(|x| x.to_string())?;

    let modified_time = metadata.modified().map_err(|x| x.to_string())?;

    let unix_timestamp = modified_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // metadata.is_dir() might be slightly more efficient than calling it on 'entry'
    let file_id = if metadata.is_dir() {
        //for directories, use the directory path since getting their ID is more difficult
        entry.to_string_lossy().to_string()
    } else {
        file_id_helper::get_file_id(entry.clone())?
    };

    let name = entry
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let dto = FileDTOInput {
        file_id,
        name,
        file_path: entry.to_string_lossy().to_string(),
        metadata: "test metadata".to_string(),
        date_modified: unix_timestamp,
        popularity: 1.0,
    };
    Ok(dto)
}
