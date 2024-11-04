use crossbeam::queue::SegQueue;
use std::{path::PathBuf, sync::Arc, time::UNIX_EPOCH};
use tokio::sync::RwLock;
use tokio::time::{self, Duration};
use tokio::{
    sync::{mpsc, Semaphore},
    task::JoinSet,
};

use crate::tantivy_file_indexer::{
    dtos::file_dto_input::FileDTOInput,
    services::search_index::models::index_worker::file_input::FileInputModel, util::file_id_helper,
};

use super::crawler_queue::CrawlerQueue;

pub async fn spawn_worker(
    sender: mpsc::Sender<FileInputModel>,
    max_concurrent_tasks: usize,
    save_queue_after: usize,
    queue: Arc<CrawlerQueue>,
) {
    let dir_entries: Arc<SegQueue<FileDTOInput>> = Arc::new(SegQueue::new());
    let semaphore = Arc::new(Semaphore::new(max_concurrent_tasks));
    let mut tasks = JoinSet::new();

    // Keep track of files to process until queue should be saved
    let files_processed: Arc<RwLock<usize>> = Arc::new(RwLock::new(0));

    let worker_queue = queue.clone();
    loop {
        // Check if we have a path to process from the queue
        let path = match queue.pop().await {
            Some(path) => path,
            None => {
                // Exit if no tasks are running and the queue is empty
                if tasks.is_empty() {
                    println!("worker done processing");
                    break;
                }
                // Sleep briefly to wait for more entries if tasks are still processing
                time::sleep(Duration::from_millis(100)).await;
                continue;
            }
        };

        let semaphore_clone = semaphore.clone();
        let queue_clone = worker_queue.clone();
        let sender_clone = sender.clone();
        let dir_entries_clone = Arc::clone(&dir_entries);
        let files_processed_clone: Arc<RwLock<usize>> = files_processed.clone();

        // Spawn the task directly into the JoinSet
        tasks.spawn(async move {
            // Use a semaphore permit for concurrency control
            let _permit = semaphore_clone
                .acquire_owned()
                .await
                .expect("Failed to acquire semaphore");

            if path.is_dir() {
                if let Ok(dir) = tokio::fs::read_dir(&path).await {
                    tokio::pin!(dir);
                    while let Ok(Some(entry)) = dir.next_entry().await {
                        let entry_path = entry.path();
                        if entry_path.is_dir() {
                            queue_clone.push_default(entry_path).await;
                        }
                        if let Ok(dto) = create_dto(&entry).await {
                            dir_entries_clone.push(dto);
                            // Increment queue save counter
                            
                            *files_processed_clone.write().await += 1;
                            if *files_processed_clone.read().await > save_queue_after {
                                *files_processed_clone.write().await = 0;
                                // Save queue
                                if let Err(err) = queue_clone.save().await {
                                    println!("Failed to save queue: {}", err);
                                }
                            }
                            
                        }
                    }
                }
            }

            let model = create_model(path, &dir_entries_clone);
            println!("sending to indexer");
            if let Err(err) = sender_clone.send(model).await {
                println!("Error sending FileInputModel to indexer: {}", err);
            }
        });

        // Process tasks as they complete to handle task management
        while let Some(result) = tasks.join_next().await {
            if let Err(err) = result {
                println!("Task error: {:?}", err);
            }
        }
    }
}

fn create_model(directory_from: PathBuf, entries: &SegQueue<FileDTOInput>) -> FileInputModel {
    let mut dtos = Vec::<FileDTOInput>::new();
    while let Some(entry) = entries.pop() {
        dtos.push(entry);
    }
    FileInputModel {
        dtos,
        directory_from,
    }
}

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
