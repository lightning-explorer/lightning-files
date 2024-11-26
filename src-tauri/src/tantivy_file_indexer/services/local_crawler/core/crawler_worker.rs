use std::time::{Duration, Instant};
use std::{path::PathBuf, sync::Arc, time::UNIX_EPOCH};
use tokio::sync::Notify;

use crate::tantivy_file_indexer::services::local_crawler::analyzer::service::FileCrawlerAnalyzerService;
use crate::tantivy_file_indexer::shared::local_db_and_search_index::traits::file_sender_receiver::FileIndexerSender;
use crate::tantivy_file_indexer::{
    dtos::file_dto_input::FileDTOInput,
    services::search_index::models::index_worker::file_input::FileInputModel, util::file_id_helper,
};

use super::crawler_queue::CrawlerQueue;

pub async fn worker_task<T>(
    sender: T,
    queue: Arc<CrawlerQueue>,
    analyzer: Option<Arc<FileCrawlerAnalyzerService>>,
    notify: Arc<Notify>,
    worker_id: u32,
) where
    T: FileIndexerSender,
{
    #[cfg(feature = "file_crawler_logs")]
    println!(
        "File crawler subworker has been spawned to process entries in directory. ID: {}",
        worker_id
    );
    loop {
        if let Some(ref analyzer) = analyzer {
            analyzer.record_timestamp().await;
        }
        /*
        Current issue:
        If 'queue.pop' errors, it returns None, which causes the crawler to become idle.
        In the case of using 'notified.await', this will cause the crawler to idle, and because it tried to take an item out of the queue,
        it will wait even though there are entries that could still be processed, thus if all of the crawlers eventually idle, then nothing
        will get crawled.

        To fix this, we need to retry calls to the database to ensure that something is found
         */
        if let Some((path, priority)) = queue.pop().await {
            let mut input_dtos_tasks = Vec::new();
            let mut dir_paths_priority: Vec<(PathBuf, u32)> = Vec::new();
            let mut files_processed: usize = 0;

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
                        let metadata_fetch_task =
                            tokio::spawn(async move { create_dto(entry_path).await });
                        input_dtos_tasks.push(metadata_fetch_task);

                        files_processed += 1;
                    }
                    // End of the while loop. Put some heavier operations here:
                    // This is a database call here:
                    queue.push_many(&dir_paths_priority).await;

                    if let Some(ref analyzer) = analyzer {
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
        } else {
            // Wait to be notified
            //notify.notified().await;
            println!("Crawler worker has nothing to do. Idling");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}

async fn create_model(directory_from: PathBuf, dtos: Vec<FileDTOInput>) -> FileInputModel {
    FileInputModel {
        dtos,
        directory_from,
    }
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
