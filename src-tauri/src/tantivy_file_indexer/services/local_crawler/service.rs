use tantivy::schema::Schema;
use tantivy::IndexWriter;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinSet;

use crate::tantivy_file_indexer::{
    services::local_db::service::LocalDbService,
    shared::local_db_and_search_index::traits::file_sender_receiver::FileIndexerSender,
};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use super::core::indexing_crawler;
use super::core::indexing_crawler::worker_manager::retry_with_backoff;
use super::{
    analyzer::service::FileCrawlerAnalyzerService,
    core::crawler_queue::queue::{CrawlerQueue, Priority},
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

    pub async fn spawn_indexing_crawlers(
        &self,
        index_writer: Arc<Mutex<IndexWriter>>,
        schema: Schema,
        worker_batch_size: usize,
    ) -> JoinSet<()> {
        let files_collection = self.local_db.files_table().clone();
        indexing_crawler::worker_manager::spawn_worker_pool(
            self.queue.clone(),
            files_collection.into(),
            (index_writer, schema),
            self.queue.get_notifier(),
            worker_batch_size,
            self.max_concurrent_tasks,
        )
        .await
    }

    pub async fn push_dirs(&self, paths: Vec<(PathBuf, Priority)>) {
        if let Err(err) = retry_with_backoff(
            || self.queue.push_many(&paths),
            4,
            Duration::from_millis(1000),
        )
        .await
        {
            println!(
                "FileCrawlerService - Failed to push directories to queue. Err: {}",
                err
            );
        }
    }

    pub async fn push_dirs_default(&self, paths: Vec<PathBuf>) {
        self.queue.push_defaults(&paths).await;
    }
}
