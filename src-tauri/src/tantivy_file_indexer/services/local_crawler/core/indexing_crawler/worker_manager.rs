use std::{sync::Arc, time::Duration};

use tokio::sync::Notify;

use super::super::crawler_queue::CrawlerQueue;

/*
Workflow:
The manager pulls a batch of directories from the crawler queue, then, that many threads will get spawned to crawl and make DTOs out of them.
then they all get indexed
*/

pub async fn manager_task(
    crawler_queue: Arc<CrawlerQueue>,
    notify: Arc<Notify>,
    max_concurrent_tasks: usize,
) {
    loop {
        let mut paths_batch = Vec::new();
        let mut none_left = false;

        // Attempt to pop from the queue
        match crawler_queue.pop().await {
            Ok(Some(path)) => paths_batch.push(path),
            Ok(None) => {
                // Queue is empty
                none_left = true;
            }
            Err(err) => {
                eprintln!(
                    "File crawler task manager encountered an error: {}. Retrying in 1 second.",
                    err
                );
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }
        }

        if paths_batch.len() >= max_concurrent_tasks || (none_left && !paths_batch.is_empty()) {
            // Process the batch of paths here
        } else if none_left {
            // If queue is empty, wait for notification
            println!("No tasks available. Waiting for notification...");
            notify.notified().await;
        }
    }
}
