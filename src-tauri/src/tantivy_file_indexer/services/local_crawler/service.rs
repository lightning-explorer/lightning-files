use tokio::sync::{mpsc, Mutex};

use crate::tantivy_file_indexer::services::local_db::service::SqlxService;
use crate::tantivy_file_indexer::services::search_index::models::index_worker::file_input::FileInputModel;
use crate::tantivy_file_indexer::services::search_index::service::SearchIndexService;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::core::crawler_queue::CrawlerQueue;

pub struct FileCrawlerService {
    max_concurrent_tasks: usize,
    queue: Arc<CrawlerQueue>,
    search_service: Arc<SearchIndexService>,
    db_service: Arc<SqlxService>,
}

impl FileCrawlerService {
    pub fn new(
        max_concurrent_tasks: usize,
        search_service: Arc<SearchIndexService>,
        db_service: Arc<SqlxService>,
    ) -> Self {
        Self {
            max_concurrent_tasks,
            search_service,
            db_service,
            queue: Arc::new(CrawlerQueue::new(vec![])),
        }
    }

    pub fn spawn_crawler(&self, sender: mpsc::Sender<FileInputModel>) {
        let queue = self.queue.clone();
        let max_concurrent_tasks = self.max_concurrent_tasks;

        tokio::task::spawn(async move {
            super::core::crawler_worker::spawn_worker(sender, max_concurrent_tasks, queue).await;
        });
    }

    pub fn push_dirs(&self, paths: Vec<&str>) {
        let dirs: Vec<PathBuf> = paths.iter().map(|x| Path::new(x).to_path_buf()).collect();
        self.process_dirs(dirs);
    }

    fn process_dirs(&self, paths: Vec<PathBuf>) {
        for path in paths {
            self.queue.push(path);
        }
    }
}
