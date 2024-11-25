use std::sync::Arc;

use tokio::sync::Notify;
use tokio::task::JoinSet;

use crate::tantivy_file_indexer::services::local_crawler::analyzer::service::FileCrawlerAnalyzerService;
use crate::tantivy_file_indexer::shared::local_db_and_search_index::traits::file_sender_receiver::FileIndexerSender;

use super::crawler_queue::CrawlerQueue;
use super::crawler_worker::worker_task;

pub fn spawn_workers<T>(
    sender: T,
    max_concurrent_tasks: usize,
    queue: Arc<CrawlerQueue>,
    notify: Arc<Notify>,
) ->JoinSet<()> where
    T: FileIndexerSender,
{
    spawn_workers_internal(sender, max_concurrent_tasks, queue, None, notify)
}

pub fn spawn_workers_with_analyzer<T>(
    sender: T,
    max_concurrent_tasks: usize,
    queue: Arc<CrawlerQueue>,
    analyzer: Arc<FileCrawlerAnalyzerService>,
    notify: Arc<Notify>,
) ->JoinSet<()> where
    T: FileIndexerSender,
{
    spawn_workers_internal(sender, max_concurrent_tasks, queue, Some(analyzer), notify)
}

pub fn spawn_workers_internal<T>(
    sender: T,
    max_concurrent_tasks: usize,
    queue: Arc<CrawlerQueue>,
    analyzer: Option<Arc<FileCrawlerAnalyzerService>>,
    notify: Arc<Notify>,
) ->JoinSet<()> where
    T: FileIndexerSender,
{
    let mut tasks = JoinSet::new();
    // Use a thread pool
    for id in 0..max_concurrent_tasks {

        #[cfg(feature = "file_crawler_logs")]
        println!(
            "File crawler worker has been spawned to process entries in directory. ID: {}",
            id
        );

        tasks.spawn(worker_task(
            sender.clone(),
            Arc::clone(&queue),
            analyzer.clone(),
            Arc::clone(&notify),
        ));
    }
    tasks
}
