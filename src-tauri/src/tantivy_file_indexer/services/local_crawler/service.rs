
use tantivy::IndexWriter;
use tokio::sync::Mutex;
use tokio::task::JoinSet;

use crate::shared::models::sys_file_model::SystemFileModel;
use crate::tantivy_file_indexer::services::local_db::service::LocalDbService;
use crate::tantivy_file_indexer::services::search_index::pipelines;
use crate::tantivy_file_indexer::services::search_index::service::SearchIndexService;
use crate::tantivy_file_indexer::shared::async_retry;
use crate::tantivy_file_indexer::shared::indexing_crawler::traits::commit_pipeline::CrawlerCommitPipeline;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use super::core::crawler_queue::queue::{CrawlerQueue, Priority};
use super::core::indexing_crawler;

pub struct FileCrawlerService {
    max_concurrent_tasks: usize,
    queue: Arc<CrawlerQueue>,
    local_db: Arc<LocalDbService>,
    search_index: Arc<SearchIndexService>,
}

impl FileCrawlerService {
    pub async fn new_async(
        max_concurrent_tasks: usize,
        local_db_service: Arc<LocalDbService>,
        search_index: Arc<SearchIndexService>,
    ) -> Self {
        let queue = Arc::new(CrawlerQueue::new(Arc::clone(&local_db_service)));
        Self {
            max_concurrent_tasks,
            queue,
            local_db: Arc::clone(&local_db_service),
            search_index: Arc::clone(&search_index),
        }
    }

    /// Spawn crawlers that treat Tantivy as simply the search index and SQLite as the main database
    ///
    /// ### NOTE:
    /// Faster, but uses much more disk space
    pub async fn spawn_indexing_crawlers_db(
        &self,
        index_writer: Arc<Mutex<IndexWriter>>,
        worker_batch_size: usize,
    ) -> JoinSet<()> {
        let pipeline = pipelines::db_tantivy_pipeline::DbTantivyPipeline::new(self.local_db.files_table().clone(), index_writer);
        //let pipeline = pipelines::tantivy_pipeline::TantivyPipeline::new( index_writer);
        self.spawn_indexing_crawlers_internal(
            worker_batch_size,
            pipeline.into(),
        )
        .await
    }

    async fn spawn_indexing_crawlers_internal<P>(
        &self,
        worker_batch_size: usize,
        pipeline: Arc<P>,
    ) -> JoinSet<()>
    where
        P: CrawlerCommitPipeline<InputModel = SystemFileModel>,
    {
        indexing_crawler::worker_manager::spawn_worker_pool(
            self.queue.clone(),
            pipeline,
            self.queue.get_notifier(),
            worker_batch_size,
            self.max_concurrent_tasks,
        )
        .await
    }

    pub async fn push_dirs(&self, paths: Vec<(PathBuf, Priority)>) {
        if let Err(err) = async_retry::retry_with_backoff(
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
