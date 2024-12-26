use super::worker;
use std::sync::Arc;
use tokio::{sync::Notify, task::JoinSet};
use super::garbage_collector::CrawlerGarbageCollector;

use crate::{
    shared::models::sys_file_model::SystemFileModel,
    tantivy_file_indexer::shared::indexing_crawler::traits::{
        commit_pipeline::CrawlerCommitPipeline, crawler_queue_api::CrawlerQueueApi,
    },
};

pub struct IndexingCrawlersBuilder<C, P>
where
    C: CrawlerQueueApi,
    P: CrawlerCommitPipeline<InputModel = SystemFileModel>,
{
    crawler_queue: Arc<C>,
    pipeline: Arc<P>,
    notify: Arc<Notify>,
    worker_batch_size: usize,
    max_concurrent_tasks: usize,
    garbage_collector: Option<Arc<CrawlerGarbageCollector>>,
}

impl<C, P> IndexingCrawlersBuilder<C, P>
where
    C: CrawlerQueueApi,
    P: CrawlerCommitPipeline<InputModel = SystemFileModel>,
{
    pub fn new(crawler_queue: Arc<C>, pipeline: Arc<P>, notify: Arc<Notify>) -> Self {
        Self {
            crawler_queue,
            pipeline,
            notify,
            worker_batch_size: 512,
            max_concurrent_tasks: 4,
            garbage_collector: None
        }
    }
    pub fn batch_size(mut self, size: usize) -> Self {
        self.worker_batch_size = size;
        self
    }
    pub fn max_num_workers(mut self, num: usize) -> Self {
        self.max_concurrent_tasks = num;
        self
    }
    pub fn with_garbage_collector(mut self, c: Arc<CrawlerGarbageCollector>) -> Self {
        self.garbage_collector = Some(c);
        self
    }
    /// Returns a handle to the crawler tasks
    pub async fn build_async(self) -> JoinSet<()> {
        if let Err(err) = self.crawler_queue.set_taken_to_false_all().await {
            println!(
                "Crawler worker manager: unable to reset taken status of all items: {}",
                err
            );
        }

        println!(
            "Spawning pool of {} file crawler indexers",
            self.max_concurrent_tasks
        );

        let mut tasks: JoinSet<()> = JoinSet::new();
        for _ in 0..self.max_concurrent_tasks {
            let mut worker = worker::IndexingCrawlerWorker::new(
                Arc::clone(&self.crawler_queue),
                Arc::clone(&self.pipeline),
                Arc::clone(&self.notify),
                self.worker_batch_size,
            );
            // Inject a garbage collctor if there is one
            if let Some(g) = &self.garbage_collector{
                let collector = Arc::clone(g);
                worker.inject_garbage_collector(collector);
            }
            tasks.spawn(async move {
                worker.worker_task().await;
            });
        }
        tasks
    }
}
