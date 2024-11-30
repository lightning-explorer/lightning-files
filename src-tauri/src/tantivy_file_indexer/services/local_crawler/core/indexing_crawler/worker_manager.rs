use std::{fmt::Display, future::Future, ops::Fn, sync::Arc, time::Duration};

use rand::Rng;
use rand_chacha::rand_core::SeedableRng;
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
pub async fn spawn_worker_pool<C, F>(
    crawler_queue: Arc<C>,
    files_collection: Arc<F>,
    tantivy: TantivyInput,
    notify: Arc<Notify>,
    max_concurrent_tasks: usize,
) -> JoinSet<()>
where
    C: CrawlerQueueApi,
    F: FilesCollectionApi,
{
    let mut tasks: JoinSet<()> = JoinSet::new();
    let (ref writer, ref schema) = tantivy;
    for _ in 0..max_concurrent_tasks {
        let worker = worker::IndexingCrawlerWorker::new(
            Arc::clone(&crawler_queue),
            Arc::clone(&files_collection),
            (Arc::clone(&writer), schema.clone()),
            Arc::clone(&notify),
        );
        tasks.spawn(async move {
            worker.worker_task().await;
        });
    }
    tasks
}

pub async fn retry_with_backoff<T, E, F, Fut>(
    function: F,
    max_retries: usize,
    initial_delay: Duration,
) -> Result<T, String>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: Display,
{
    let mut delay = initial_delay;
    // Use a thread-safe RNG
    let mut rng = rand_chacha::ChaChaRng::from_entropy();

    for attempt in 1..=max_retries {
        match function().await {
            Ok(result) => return Ok(result),
            Err(_) if attempt < max_retries => {
                // Add jitter: Randomize delay within 50%-150% of the current delay
                let jitter: f64 = rng.gen_range(0.5..1.5);
                let jittered_delay = delay.mul_f64(jitter);

                tokio::time::sleep(jittered_delay).await;

                // Exponential backoff: Double the delay for the next attempt
                delay = delay * 2;
            }
            Err(err) => {
                return Err(format!(
                    "Function failed after {} attempts. Last error: {}",
                    max_retries, err
                ));
            }
        }
    }

    unreachable!() // This should never be reached because all cases are handled above.
}
