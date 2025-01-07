use tokio::{sync::mpsc, task::JoinHandle};

use crate::tantivy_file_indexer::services::{
    local_crawler::core::crawler_queue::queue::CrawlerQueue,
    search_index::pipelines::tantivy_pipeline::TantivyPipeline,
};

use super::{builder, worker_task_handle::CrawlerWorkerTaskHandle};

/// A message from the crawler task manager
pub enum CrawlerMessage {
    Kill,
    Throttle(u32),
}
pub type CrawlerManagerMessageReceiver = mpsc::Receiver<CrawlerMessage>;
pub type CrawlerManagerMessageSender = mpsc::Sender<CrawlerMessage>;

pub async fn build_managed(
    builder: builder::IndexingCrawlersBuilder<CrawlerQueue, TantivyPipeline>,
) {
    let num_workers = 2;
    let tasks = builder
        .max_num_workers(num_workers)
        .batch_size(512)
        .build()
        .await;

    println!(
        "Crawler task manager has spawned {} file crawlers",
        num_workers
    );
    manage_crawl_tasks(tasks);
}
fn manage_crawl_tasks(mut crawl_task_handles: Vec<CrawlerWorkerTaskHandle>) {
    let senders: Vec<CrawlerManagerMessageSender> = crawl_task_handles
        .iter_mut()
        .map(|x| x.take_sender())
        .collect();
    let handles: Vec<JoinHandle<()>> = crawl_task_handles
        .into_iter()
        .map(|x| x.take_handle())
        .collect();

    // TODO: have the task manager actually do stuff
    tokio::spawn(async move {
        for handle in handles {
            handle.await;
        }
        drop(senders);
    });
}
