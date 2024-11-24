
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
) where T: FileIndexerSender<FileInputModel> {
    spawn_worker_internal(sender, max_concurrent_tasks, queue, None).await;
}

pub async fn spawn_worker_with_analyzer<T>(
    sender: T,
    max_concurrent_tasks: usize,
    queue: Arc<CrawlerQueue>,
    analyzer: Arc<FileCrawlerAnalyzerService>,
) where T: FileIndexerSender<FileInputModel> {
    spawn_worker_internal(sender, max_concurrent_tasks, queue, Some(analyzer)).await;
}

// Note that the crawler does not handle database operations
pub async fn spawn_worker_internal<T>(
    sender: T,
    max_concurrent_tasks: usize,
    queue: Arc<CrawlerQueue>,
    analyzer: Option<Arc<FileCrawlerAnalyzerService>>,
) where T: FileIndexerSender<FileInputModel> {
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
            let semaphore = Arc::clone(&semaphore);
            let sender = sender.clone();
            let queue = Arc::clone(&worker_queue);
            let analyzer_clone = analyzer.clone();

            subworker_id += 1;

            tasks.spawn(async move {

                #[cfg(feature = "file_crawler_logs")]
                println!(
                    "File crawler subworker has been spawned to process entries in directory. ID: {}",
                    subworker_id
                );
            

                let mut dir_entries = Vec::new();

                let _permit = semaphore
                    .acquire_owned()
                    .await
                    .expect("Failed to acquire semaphore");

                if path.is_dir() {
                    if let Ok(mut dir) = tokio::fs::read_dir(&path).await {
                        while let Ok(Some(entry)) = dir.next_entry().await {
                            let entry_path = entry.path();
                            if entry_path.is_dir() {
                                queue.push(entry_path.clone(), priority + 1).await;
                            }

                            //if let Ok(dto) = create_dto(&entry).await {
                            dir_entries.push(entry_path);
                            //}

                            // A directory or a file counts as a file being processed
                            if let Some(ref analyzer) = analyzer_clone {
                                analyzer.add_one_to_files_processed();
                            }
                        }
                    }
                }

                #[cfg(feature = "speed_profile")]
                let time = Instant::now();

                // DTOs get created here
                let model = create_model(path, dir_entries).await;

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

async fn create_model(directory_from: PathBuf, entries: Vec<PathBuf>) -> FileInputModel {
    let dtos: Vec<FileDTOInput> = create_dtos_batch(entries)
        .await
        .into_iter()
        .flatten()
        .collect();
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
