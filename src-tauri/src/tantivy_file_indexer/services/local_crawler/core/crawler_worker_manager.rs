use std::sync::Arc;

use tokio::sync::Notify;

use crate::tantivy_file_indexer::services::local_crawler::analyzer::service::FileCrawlerAnalyzerService;
use crate::tantivy_file_indexer::shared::local_db_and_search_index::traits::file_sender_receiver::FileIndexerSender;

use super::crawler_queue::CrawlerQueue;
use super::crawler_worker::worker_task;

pub async fn spawn_workers<T>(
    sender: T,
    max_concurrent_tasks: usize,
    queue: Arc<CrawlerQueue>,
    notify: Arc<Notify>,
) where
    T: FileIndexerSender,
{
    spawn_workers_internal(sender, max_concurrent_tasks, queue, None, notify).await;
}

pub async fn spawn_workers_with_analyzer<T>(
    sender: T,
    max_concurrent_tasks: usize,
    queue: Arc<CrawlerQueue>,
    analyzer: Arc<FileCrawlerAnalyzerService>,
    notify: Arc<Notify>,
) where
    T: FileIndexerSender,
{
    spawn_workers_internal(sender, max_concurrent_tasks, queue, Some(analyzer), notify).await;
}

pub async fn spawn_workers_internal<T>(
    sender: T,
    max_concurrent_tasks: usize,
    queue: Arc<CrawlerQueue>,
    analyzer: Option<Arc<FileCrawlerAnalyzerService>>,
    notify: Arc<Notify>,
) where
    T: FileIndexerSender,
{
    // Use a thread pool
    for id in 0..max_concurrent_tasks {
        tokio::spawn(worker_task(
            sender.clone(),
            Arc::clone(&queue),
            analyzer.clone(),
            Arc::clone(&notify),
            id as u32,
        ));
    }
}
