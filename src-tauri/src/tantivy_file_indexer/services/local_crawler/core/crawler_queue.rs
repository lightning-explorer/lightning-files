use std::{path::PathBuf, sync::Arc};

use tokio::sync::RwLock;

use crate::{
    shared::collections::popularity_set::PopularitySet,
    tantivy_file_indexer::services::app_save::service::AppSaveService,
};

const DEFAULT_PRIORITY: Priority = 1;
const SAVE_NAME: &str = "files_queue";
pub struct CrawlerQueue {
    queue: Arc<RwLock<PopularitySet<PathBuf>>>,
    save_service: Arc<AppSaveService>,
}

pub type Priority = u32;
impl CrawlerQueue {
    /**
     * An 'iteration' is one file being processed
     */
    pub async fn new_async(directories: Vec<PathBuf>, save_service: Arc<AppSaveService>) -> Self {
        let queue = Arc::new(RwLock::new(PopularitySet::<PathBuf>::new()));
        for item in directories {
            queue.write().await.insert(item, DEFAULT_PRIORITY);
        }
        Self {
            save_service,
            queue,
        }
    }

    pub async fn push(&self, directory: PathBuf, priority: Priority) {
        self.queue.write().await.insert(directory, priority);
    }

    pub async fn push_default(&self, directory: PathBuf) {
        self.queue.write().await.insert(directory, DEFAULT_PRIORITY);
    }

    pub async fn pop(&self) -> Option<PathBuf> {
        self.queue.write().await.pop()
    }

    pub async fn get_len(&self) -> usize {
        self.queue.read().await.len()
    }

    pub async fn save(&self) -> Result<(), std::io::Error> {
        self.save_service.save(SAVE_NAME, self.queue_as_vec().await)
    }

    /**
     * Fallback in case the save path doesn't exist
     */
    pub async fn load_or(&self, fallback_directories: Vec<PathBuf>) {
        if self.load().await.is_err() {
            self.populate_queue(fallback_directories).await;
        }
    }

    pub async fn load(&self) -> Result<(), std::io::Error> {
        let entries = self.save_service.load::<Vec<PathBuf>>(SAVE_NAME)?;
        self.populate_queue(entries).await;
        Ok(())
    }

    async fn populate_queue(&self, entries: Vec<PathBuf>) {
        self.queue
            .write()
            .await
            .insert_many(entries.into_iter().map(|x| (x, DEFAULT_PRIORITY)).collect());
    }

    async fn queue_as_vec(&self) -> Vec<PathBuf> {
        self.queue.read().await.as_partial_vec()
    }
}
