use tokio::sync::RwLock;

use crate::tantivy_file_indexer::services::app_save::service::AppSaveService;
use crate::tantivy_file_indexer::services::local_db::service::LocalDbService;
use crate::tantivy_file_indexer::services::search_index::service::SearchIndexService;
use crate::tantivy_file_indexer::shared::async_retry;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use super::core::crawler_queue::queue::{CrawlerQueue, Priority};
use super::core::indexing_crawler::plugins::filterer::CrawlerFilterer;
use super::core::indexing_crawler::task_manager;
use super::core::indexing_crawler::{builder, plugins::garbage_collector};

pub struct FileCrawlerService {
    /// True if the file crawlers are already crawling around
    has_dispatched_crawlers:RwLock<bool>,

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
            has_dispatched_crawlers: RwLock::new(false),

            queue,
            search_index: Arc::clone(&search_index),
            local_db_service,
            save_service,
        }
    }

    /// Once built, the crawlers will get dispatched and start working
    /// 
    /// If this function gets called while the crawlers are already dispatched, nothing will happen
    pub async fn dispatch_crawlers(&self) {
        let mut has_dispatched_crawlers_lock = self.has_dispatched_crawlers.write().await;
        if *has_dispatched_crawlers_lock{
            return;
        }
        *has_dispatched_crawlers_lock = true;

        let pipeline = self.search_index.get_pipeline();
        let crawler_queue = Arc::clone(&self.queue);

        // Create the garbage collector and inject it
        let collector = Arc::new(garbage_collector::CrawlerGarbageCollector::new(
            Arc::clone(&self.local_db_service),
            Arc::clone(&self.save_service),
            Arc::clone(&self.search_index),
        ));

        let filterer = Arc::new(CrawlerFilterer::new(
            self.local_db_service.kv_store_table().clone()
        ));

        let builder = builder::IndexingCrawlersBuilder::new(crawler_queue, pipeline)
            .with_garbage_collector(collector)
            .with_filterer(filterer);

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
}
