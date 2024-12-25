use std::sync::Arc;

use tokio::{sync::Notify, task::JoinSet};

use crate::{
    shared::models::sys_file_model::SystemFileModel,
    tantivy_file_indexer::shared::indexing_crawler::traits::{
        commit_pipeline::CrawlerCommitPipeline, crawler_queue_api::CrawlerQueueApi,
    },
};

use super::worker;

/// Returns the handles to the workers that were spawned
///
/// Because an intial database call is made, this function must be awaited.
pub async fn spawn_worker_pool<C, P>(
    crawler_queue: Arc<C>,
    pipeline: Arc<P>,
    notify: Arc<Notify>,
    worker_batch_size: usize,
    max_concurrent_tasks: usize,
) -> JoinSet<()>
where
    C: CrawlerQueueApi,
    P: CrawlerCommitPipeline<InputModel = SystemFileModel>,
{
    // Because a new session has started, all of the items in the queue are fair game
    if let Err(err) = crawler_queue.set_taken_to_false_all().await {
        println!(
            "Crawler worker manager: unable to reset taken status of all items: {}",
            err
        );
    }

    println!(
        "Spawning pool of {} file crawler indexers",
        max_concurrent_tasks
    );

    let mut tasks: JoinSet<()> = JoinSet::new();
    for _ in 0..max_concurrent_tasks {
        let worker = worker::IndexingCrawlerWorker::new(
            Arc::clone(&crawler_queue),
            Arc::clone(&pipeline),
            Arc::clone(&notify),
            worker_batch_size,
        );
        tasks.spawn(async move {
            worker.worker_task().await;
        });
    }
    tasks
}
