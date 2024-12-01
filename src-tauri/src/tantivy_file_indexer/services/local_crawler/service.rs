use tokio::sync::mpsc;

use crate::tantivy_file_indexer::services::search_index::models::index_worker::file_input::FileInputModel;
use crate::tantivy_file_indexer::{
    services::local_db::service::LocalDbService,
    shared::local_db_and_search_index::traits::file_sender_receiver::FileIndexerSender,
};
use std::path::PathBuf;
use std::sync::Arc;

use super::core::indexing_crawler;
use super::{
    analyzer::service::FileCrawlerAnalyzerService,
    core::crawler_queue::{CrawlerQueue, Priority},
};

pub struct FileCrawlerService {
    max_concurrent_tasks: usize,
    queue: Arc<CrawlerQueue>,
    local_db: Arc<LocalDbService>,
}

impl FileCrawlerService {
    pub async fn new_async(
        max_concurrent_tasks: usize,
        local_db_service: Arc<LocalDbService>,
    ) -> Self {
        let queue = Arc::new(CrawlerQueue::new(Arc::clone(&local_db_service)));
        Self {
            max_concurrent_tasks,
            queue,
            local_db: Arc::clone(&local_db_service),
        }
    }

    pub fn spawn_crawler<T>(&self, sender: T)
    where
        T: FileIndexerSender,
    {
        let queue = self.queue.clone();
        let max_concurrent_tasks = self.max_concurrent_tasks;
        let notify = self.queue.get_notifier();

        tokio::task::spawn(async move {
            super::core::file_crawler::crawler_worker_manager::spawn_workers(
                sender,
                max_concurrent_tasks,
                queue,
                notify,
            )
            .await;
        });
    }

    pub fn spawn_crawler_with_analyzer<T>(
        &self,
        sender: T,
        analyzer: Arc<FileCrawlerAnalyzerService>,
    ) where
        T: FileIndexerSender,
    {
        let queue = self.queue.clone();
        let max_concurrent_tasks = self.max_concurrent_tasks;
        let notify = self.queue.get_notifier();

        tokio::task::spawn(async move {
            super::core::file_crawler::crawler_worker_manager::spawn_workers_with_analyzer(
                sender,
                max_concurrent_tasks,
                queue,
                analyzer,
                notify,
            )
            .await;
        });
    }

    pub fn spawn_indexing_crawlers(&self) {
        let crawler_queue = self.local_db.crawler_queue_table().clone();
        let files_collection = self.local_db.files_table().clone();
        indexing_crawler::worker_manager::spawn_worker_pool(
            crawler_queue.into(),
            files_collection.into(),
            tantivy,
            notify,
            512,
            8,
        );
    }

    pub async fn push_dirs(&self, paths: Vec<(PathBuf, Priority)>) {
        self.queue.push_many(&paths).await;
    }

    pub async fn push_dirs_default(&self, paths: Vec<PathBuf>) {
        self.queue.push_defaults(&paths).await;
    }
}
