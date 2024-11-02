use std::{
    fs::{self, DirEntry},
    path::PathBuf,
    sync::Arc,
    time::UNIX_EPOCH,
};

use crossbeam::{channel::bounded, queue::SegQueue};
use tokio::sync::mpsc;

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
    // Create a queue to hold the directories to be processed concurrently
    let dir_entries: Arc<SegQueue<FileDTOInput>> = Arc::new(SegQueue::new());

    // Use a channel with a bounded capacity to limit concurrent tasks
    let (tx, rx) = bounded::<()>(max_concurrent_tasks);

    let mut tasks = Vec::new();
    while let Some(path) = queue.pop() {
        let queue_clone = queue.clone();
        let path_clone = path.clone();
        let sender_clone = sender.clone();
        let dir_entries_clone = Arc::clone(&dir_entries);
        let tx_clone = tx.clone();
        let rx_clone = rx.clone();

        // Spawn each task to handle the processing of a path concurrently
        let task = tokio::task::spawn(async move {
            let _permit = tx_clone.send(()); // Acquire a "permit" for concurrency control
            if path_clone.is_dir() {
                if let Ok(dir) = fs::read_dir(&path_clone) {
                    for entry in dir {
                        match entry {
                            Ok(entry) => {
                                let entry_path = entry.path();
                                if entry_path.is_dir() {
                                    queue_clone.push(entry_path);
                                }
                                if let Ok(dto) = create_dto(&entry) {
                                    dir_entries_clone.push(dto);
                                }
                            }
                            Err(err) => {
                                println!("Error reading directory: {}", err);
                            }
                        }
                    }
                }
            }

            // All paths in the directory have been read
            let model = create_model(path_clone, &dir_entries_clone);
            if let Err(err) = sender_clone.send(model).await {
                println!("Error sending FileInputModel to indexer: {}", err);
            }
            if let Err(err) = rx_clone.recv() {
                println!("Permit failed to release: {}", err);
            }
        });
        tasks.push(task);
    }

    // Await all tasks to complete
    futures::future::join_all(tasks).await;
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

fn create_dto(entry: &DirEntry) -> Result<FileDTOInput, String> {
    let metadata = entry.metadata().map_err(|x| x.to_string())?;

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
