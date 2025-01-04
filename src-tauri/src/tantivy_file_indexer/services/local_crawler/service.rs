use crate::tantivy_file_indexer::services::app_save::service::AppSaveService;
use crate::tantivy_file_indexer::services::local_db::service::LocalDbService;
use crate::tantivy_file_indexer::services::search_index::service::SearchIndexService;
use crate::tantivy_file_indexer::shared::async_retry;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use super::core::crawler_queue::queue::{CrawlerQueue, Priority};
use super::core::indexing_crawler::task_manager;
use super::core::indexing_crawler::{builder, plugins::garbage_collector};

pub struct FileCrawlerService {
    queue: Arc<CrawlerQueue>,
    search_index: Arc<SearchIndexService>,
    local_db_service: Arc<LocalDbService>,
    save_service: Arc<AppSaveService>,
}

impl FileCrawlerService {
    pub async fn new_async(
        local_db_service: Arc<LocalDbService>,
        search_index: Arc<SearchIndexService>,
        save_service: Arc<AppSaveService>,
    ) -> Self {
        let queue = Arc::new(CrawlerQueue::new(Arc::clone(&local_db_service)));
        Self {
            queue,
            search_index: Arc::clone(&search_index),
            local_db_service,
            save_service,
        }
    }

    /// Once built, the crawlers will get dispatched and start working
    pub async fn dispatch_crawlers(&self) {
        let pipeline = self.search_index.get_pipeline();
        let crawler_queue = Arc::clone(&self.queue);
        let notify = self.queue.get_notifier();

        // Create the garbage collector and inject it
        let collector = Arc::new(garbage_collector::CrawlerGarbageCollector::new(
            Arc::clone(&self.local_db_service),
            Arc::clone(&self.save_service),
            Arc::clone(&self.search_index),
        ));

        let builder = builder::IndexingCrawlersBuilder::new(crawler_queue, pipeline, notify)
            .with_garbage_collector(collector);

        // Hand off the rest of the building to the task manager
        task_manager::build_managed(builder).await;
    }

    pub async fn push_dirs(&self, paths: Vec<(PathBuf, Priority)>) {
        if let Err(err) = async_retry::retry_with_backoff(
            |_| self.queue.push_many(&paths),
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
