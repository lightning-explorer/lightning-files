use tokio::sync::mpsc;

use crate::tantivy_file_indexer::services::local_db::service::LocalDbService;
use crate::tantivy_file_indexer::services::search_index::models::index_worker::file_input::FileInputModel;
use std::path::PathBuf;
use std::sync::Arc;

use super::core::crawler_queue::{CrawlerQueue, Priority};

pub struct FileCrawlerService {
    max_concurrent_tasks: usize,
    crawler_save_after_iters: usize,
    queue: Arc<CrawlerQueue>,
}

impl FileCrawlerService {
    pub async fn new_async(
        max_concurrent_tasks: usize,
        crawler_save_after_iters: usize,
        local_db_service: Arc<LocalDbService>,
    ) -> Self {
        let queue = Arc::new(CrawlerQueue::new(Arc::clone(&local_db_service)));
        Self {
            max_concurrent_tasks,
            crawler_save_after_iters,
            queue,
        }
    }

    pub fn spawn_crawler(&self, sender: mpsc::Sender<FileInputModel>) {
        let queue = self.queue.clone();
        let max_concurrent_tasks = self.max_concurrent_tasks;
        let crawler_save_after_iters = self.crawler_save_after_iters;

        tokio::task::spawn(async move {
            super::core::crawler_worker::spawn_worker(
                sender,
                max_concurrent_tasks,
                crawler_save_after_iters,
                queue,
            )
            .await;
        });
    }

    pub async fn push_dirs(&self, paths: Vec<(PathBuf, Priority)>) {
        self.queue.push_many(&paths).await;
    }

    pub async fn push_dirs_default(&self, paths: Vec<PathBuf>) {
        self.queue.push_defaults(&paths).await;
    }
}
