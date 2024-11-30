use std::{sync::Arc, time::Duration};

use tokio::sync::Notify;

use crate::tantivy_file_indexer::{dtos::file_dto_input::FileDTOInput, shared::indexing_crawler::{models::crawler_file::CrawlerFile, traits::{crawler_queue_api::CrawlerQueueApi, files_collection_api::FilesCollectionApi}}};

use super::{
    crawler::{self, CrawlerError},
    indexer,
    worker_manager::{retry_with_backoff, TantivyInput},
};

pub struct IndexingCrawlerWorker<C, F>
where
    C: CrawlerQueueApi,
    F: FilesCollectionApi,
{
    crawler_queue: Arc<C>,
    files_collection: Arc<F>,
    tantivy: TantivyInput,
    notify: Arc<Notify>,
}

impl<C, F> IndexingCrawlerWorker<C, F>
where
    C: CrawlerQueueApi,
    F: FilesCollectionApi,
{
    /// Note that `worker_task` must be called in order for the background operations to start
    pub fn new(
        crawler_queue: Arc<C>,
        files_collection: Arc<F>,
        tantivy: TantivyInput,
        notify: Arc<Notify>,
    ) -> Self {
        Self {
            crawler_queue,
            files_collection,
            tantivy,
            notify,
        }
    }

    pub async fn worker_task(&self) {
        loop {
            match self.crawler_queue.fetch_next().await {
                Ok(file_option) => match file_option {
                    Some(file) => {
                        let dtos = self.handle_crawl(&file).await;
                        // If the operation was unsuccessful, then 'dtos' will be empty
                        if !dtos.is_empty() {
                            self.handle_index(&file, &dtos).await;
                        }
                    }
                    None => {
                        println!("No tasks available. Waiting for notification...");
                        self.notify.notified().await;
                    }
                },
                Err(err) => {
                    eprintln!(
                    "File crawler task encountered an error trying to fetch item from queue: {}. Retrying in 1 second.",
                    err
                );
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }
            }
        }
    }

    async fn handle_index(&self, file: &CrawlerFile, dtos: &Vec<FileDTOInput>) {
        let (ref writer, ref schema) = self.tantivy;
        match retry_with_backoff(
            || {
                indexer::index_files(
                    dtos,
                    (Arc::clone(writer), schema.clone()),
                    file.path.clone(),
                    Arc::clone(&self.files_collection),
                )
            },
            5,
            Duration::from_millis(1000),
        )
        .await
        {
            Ok(_) => {
                // If all goes well, then the directory can be removed from the crawler queue
                if let Err(err) = self.remove_file_from_queue(file).await {
                    // If for some reason the directory can't be removed, it is no big deal, it just means
                    // that it will get indexed again
                    println!(
                        "Error trying to remove directory from crawler queue: {}",
                        err
                    );
                }
            }
            Err(err) => {
                println!("Error indexing files: {}", err);
            }
        }
    }

    async fn handle_crawl(&self, file: &CrawlerFile) -> Vec<FileDTOInput>
    where
        C: CrawlerQueueApi,
    {
        match crawler::crawl(file, Arc::clone(&self.crawler_queue)).await {
            Ok(dtos) => {
                return dtos;
            }
            Err(err) => match err {
                CrawlerError::PushToQueueError(err) => {
                    println!(
                        "Crawler could not push found directories to queue: {}. 
            The original directory will stay in the queue for re-indexing",
                        err
                    );
                }
                CrawlerError::ReadDirError(err) => {
                    println!(
                        "Crawler could not read directory: {}. Removing it from the queue",
                        err
                    );
                    if let Err(err) = self.remove_file_from_queue(file).await {
                        println!(
                            "Error trying to remove directory from crawler queue: {}",
                            err
                        );
                    }
                }
            },
        }
        Vec::new()
    }

    async fn remove_file_from_queue(&self, file: &CrawlerFile) -> Result<(), String> {
        retry_with_backoff(
            || self.crawler_queue.delete_one(file.clone()),
            5,
            Duration::from_millis(1000),
        )
        .await
    }
}
