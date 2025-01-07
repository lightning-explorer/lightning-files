use super::plugins::filterer::CrawlerFilterer;
use super::plugins::garbage_collector::CrawlerGarbageCollector;
use super::{worker, worker_task_handle::CrawlerWorkerTaskHandle};
use std::sync::Arc;
use tokio::sync::mpsc;

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
    worker_batch_size: usize,
    max_concurrent_tasks: usize,
    garbage_collector: Option<Arc<CrawlerGarbageCollector>>,
    filterer: Option<Arc<CrawlerFilterer>>,
}

impl<C, P> IndexingCrawlersBuilder<C, P>
where
    C: CrawlerQueueApi,
    P: CrawlerCommitPipeline<InputModel = SystemFileModel>,
{
    pub fn new(crawler_queue: Arc<C>, pipeline: Arc<P>) -> Self {
        Self {
            crawler_queue,
            pipeline,
            worker_batch_size: 512,
            max_concurrent_tasks: 4,
            garbage_collector: None,
            filterer: None,
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
    pub fn with_filterer(mut self, f: Arc<CrawlerFilterer>) -> Self {
        self.filterer = Some(f);
        self
    }
    /// Returns a handle to the crawler tasks
    pub async fn build(self) -> Vec<CrawlerWorkerTaskHandle> {
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

        let mut handles = Vec::new();
        for _ in 0..self.max_concurrent_tasks {
            let (sender, receiver) = mpsc::channel(10);

            let mut worker = worker::IndexingCrawlerWorker::new(
                Arc::clone(&self.crawler_queue),
                Arc::clone(&self.pipeline),
                self.worker_batch_size,receiver
            );

            // Inject a garbage collector if there is one
            if let Some(g) = &self.garbage_collector {
                let collector = Arc::clone(g);
                worker.inject_garbage_collector(collector);
            }

            // Inject a filterer if there is one
            if let Some(f) = &self.filterer {
                let filterer = Arc::clone(f);
                worker.inject_filterer(filterer);
            }

            let task = tokio::spawn(async move {
                worker.worker_task().await;
            });
            handles.push(CrawlerWorkerTaskHandle::new(sender, task));
        }
        handles
    }
}
