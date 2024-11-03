use std::{
    fs::{self, DirEntry},
    path::PathBuf,
    sync::Arc,
    time::UNIX_EPOCH,
};

use crossbeam::{channel::bounded, queue::SegQueue};
use futures::{stream::FuturesUnordered, StreamExt};
use tokio::{
    sync::{mpsc, Notify, Semaphore},
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
    queue: Arc<CrawlerQueue>,
) {
    let dir_entries: Arc<SegQueue<FileDTOInput>> = Arc::new(SegQueue::new());
    let semaphore = Arc::new(Semaphore::new(max_concurrent_tasks));
    let notify = Arc::new(Notify::new());
    let mut tasks = JoinSet::new();

    let worker_notify = notify.clone();
    let worker_queue = queue.clone();

    loop {
        // Check if we have a path to process from the queue
        if let Some(path) = worker_queue.pop().await {
            let semaphore_clone = semaphore.clone();
            let queue_clone = worker_queue.clone();
            let sender_clone = sender.clone();
            let dir_entries_clone = Arc::clone(&dir_entries);
            let notify_clone = notify.clone();

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
                                queue_clone.push(entry_path);
                                notify_clone.notify_one(); // Notify that a new item is in the queue
                            }
                            if let Ok(dto) = create_dto(&entry).await {
                                dir_entries_clone.push(dto);
                            }
                        }
                    }
                }

                let model = create_model(path, &dir_entries_clone);
                if let Err(err) = sender_clone.send(model).await {
                    println!("Error sending FileInputModel to indexer: {}", err);
                }
            });
        } else {
            // Wait for notification if the queue is empty
            worker_notify.notified().await;
        }

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
