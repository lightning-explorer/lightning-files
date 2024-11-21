use crossbeam::queue::SegQueue;
use std::time::Instant;
use std::{path::PathBuf, sync::Arc, time::UNIX_EPOCH};
use tokio::time::{self, Duration};
use tokio::{
    sync::{mpsc, Semaphore},
    task::JoinSet,
};

use crate::tantivy_file_indexer::services::local_crawler::analyzer::service::FileCrawlerAnalyzerService;
use crate::tantivy_file_indexer::{
    dtos::file_dto_input::FileDTOInput,
    services::search_index::models::index_worker::file_input::FileInputModel, util::file_id_helper,
};

use super::crawler_queue::CrawlerQueue;

pub async fn spawn_worker(
    sender: mpsc::Sender<FileInputModel>,
    max_concurrent_tasks: usize,
    queue: Arc<CrawlerQueue>,
) {
    spawn_worker_internal(sender, max_concurrent_tasks, queue, None).await;
}

pub async fn spawn_worker_with_analyzer(
    sender: mpsc::Sender<FileInputModel>,
    max_concurrent_tasks: usize,
    queue: Arc<CrawlerQueue>,
    analyzer: Arc<FileCrawlerAnalyzerService>,
) {
    spawn_worker_internal(sender, max_concurrent_tasks, queue, Some(analyzer)).await;
}

// Note that the crawler does not handle database operations
pub async fn spawn_worker_internal(
    sender: mpsc::Sender<FileInputModel>,
    max_concurrent_tasks: usize,
    queue: Arc<CrawlerQueue>,
    analyzer: Option<Arc<FileCrawlerAnalyzerService>>,
) {
    let dir_entries = Arc::new(SegQueue::new());
    let semaphore = Arc::new(Semaphore::new(max_concurrent_tasks));
    let mut tasks = JoinSet::new();
    let worker_queue = Arc::clone(&queue);

    loop {
        if let Some(ref analyzer) = analyzer {
            analyzer.record_timestamp().await;
        }
        if let Some((path, priority)) = queue.pop().await {
            let dir_entries = Arc::clone(&dir_entries);
            let semaphore = Arc::clone(&semaphore);
            let sender = sender.clone();
            let queue = Arc::clone(&worker_queue);
            let analyzer_clone = analyzer.clone();

            tasks.spawn(async move {
                let _permit = semaphore
                    .acquire_owned()
                    .await
                    .expect("Failed to acquire semaphore");

                if path.is_dir() {
                    if let Ok(mut dir) = tokio::fs::read_dir(&path).await {
                        while let Ok(Some(entry)) = dir.next_entry().await {
                            let entry_path = entry.path();
                            if entry_path.is_dir() {
                                queue.push(entry_path, priority + 1).await;
                            }
                            if let Ok(dto) = create_dto(&entry).await {
                                dir_entries.push(dto);
                            }
                            // A directory or a file counts as a file being processed
                            if let Some(ref analyzer) = analyzer_clone {
                                analyzer.add_one_to_files_processed();
                            }
                        }
                    }
                }

                let model = create_model(path, &dir_entries).await;

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
            });
        } else {
            time::sleep(Duration::from_millis(100)).await;
        }

        while let Some(result) = tasks.join_next().await {
            if let Err(err) = result {
                eprintln!("Task error: {:?}", err);
            }
        }
    }
}

async fn create_model(directory_from: PathBuf, entries: &SegQueue<FileDTOInput>) -> FileInputModel {
    let mut dtos = Vec::<FileDTOInput>::new();
    while let Some(entry) = entries.pop() {
        dtos.push(entry);
    }
    FileInputModel {
        dtos,
        directory_from,
    }
}

// This function takes around 60ms to complete so look at this
async fn create_dto(entry: &tokio::fs::DirEntry) -> Result<FileDTOInput, String> {
    let metadata = entry.metadata().await.map_err(|x| x.to_string())?;

    let modified_time = metadata.modified().map_err(|x| x.to_string())?;

    let unix_timestamp = modified_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let file_id = if entry.path().is_dir() {
        //for directories, use the directory path since getting their ID is more difficult
        entry.path().to_string_lossy().to_string()
    } else {
        file_id_helper::get_file_id(entry.path().to_path_buf())?
    };

    let dto = FileDTOInput {
        file_id,
        name: entry.file_name().to_string_lossy().to_string(),
        file_path: entry.path().to_string_lossy().to_string(),
        metadata: "test metadata".to_string(),
        date_modified: unix_timestamp,
        popularity: 1.0,
    };
    Ok(dto)
}
