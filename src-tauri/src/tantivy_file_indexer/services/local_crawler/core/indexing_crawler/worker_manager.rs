use std::sync::Arc;

use tantivy::{schema::Schema, IndexWriter};
use tokio::{
    sync::{Mutex, Notify},
    task::JoinSet,
};

use crate::tantivy_file_indexer::shared::indexing_crawler::traits::{
    crawler_queue_api::CrawlerQueueApi, files_collection_api::FilesCollectionApi,
};

use super::worker;

pub type TantivyInput = (Arc<Mutex<IndexWriter>>, Schema);

/// Returns the handles to the workers that were spawned
///
/// Because an intial database call is made, this function must be awaited.
pub async fn spawn_worker_pool<C, F>(
    crawler_queue: Arc<C>,
    files_collection: Arc<F>,
    tantivy: TantivyInput,
    notify: Arc<Notify>,
    worker_batch_size: usize,
    max_concurrent_tasks: usize,
) -> JoinSet<()>
where
    C: CrawlerQueueApi,
    F: FilesCollectionApi,
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
    let (ref writer, ref schema) = tantivy;
    for _ in 0..max_concurrent_tasks {
        let worker = worker::IndexingCrawlerWorker::new(
            Arc::clone(&crawler_queue),
            Arc::clone(&files_collection),
            (Arc::clone(writer), schema.clone()),
            Arc::clone(&notify),
            worker_batch_size,
        );
        tasks.spawn(async move {
            worker.worker_task().await;
        });
    }
    tasks
}
