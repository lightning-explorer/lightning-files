use std::{path::PathBuf, sync::Arc};

use tokio::sync::{Mutex, RwLock};

use crate::{shared::collections::popularity_set::PopularitySet, tantivy_file_indexer::app_data};

const DEFAULT_PRIORITY: u32 = 1;
const SAVE_PATH: &str = "files_queue";
pub struct CrawlerQueue {
    queue: Arc<RwLock<PopularitySet<PathBuf>>>,
    iter_on: Arc<Mutex<usize>>,
    save_after_iters: usize,
}

impl CrawlerQueue {
    /**
     * An 'iteration' is one file being processed. So `save_after_iters` should be a pretty sizeable value
     */
    pub fn new(directories: Vec<PathBuf>, save_after_iters: usize) -> Self {
        let queue = Arc::new(RwLock::new(PopularitySet::<PathBuf>::new()));
        for item in directories {
            queue.blocking_write().insert(item, DEFAULT_PRIORITY);
        }
        Self {
            queue,
            iter_on: Arc::new(Mutex::new(0)),
            save_after_iters,
        }
    }

    pub async fn push(&self, directory: PathBuf, priority: u32) {
        self.queue.write().await.insert(directory, priority);
    }

    pub async fn push_default(&self, directory: PathBuf) {
        self.queue.write().await.insert(directory, DEFAULT_PRIORITY);
    }

    pub async fn pop(&self) -> Option<PathBuf> {
        self.increment_iter().await;
        self.queue.write().await.pop()
    }

    pub async fn get_len(&self) -> usize {
        self.queue.read().await.len()
    }

    pub async fn save(&self) -> Result<(), std::io::Error> {
        app_data::json::save(SAVE_PATH, self.queue_as_vec().await)
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
        let entries = app_data::json::load::<Vec<PathBuf>>(SAVE_PATH)?;
        self.populate_queue(entries).await;
        Ok(())
    }

    async fn increment_iter(&self) {
        let mut iter_on = self.iter_on.lock().await;
        if *iter_on >= self.save_after_iters {
            *iter_on = 0;
            if let Err(err) = self.save().await {
                println!("Failed to save crawler queue: {}", err);
            }
        } else {
            *iter_on += 1;
        }
    }

    async fn populate_queue(&self, entries: Vec<PathBuf>) {
        for entry in entries.into_iter() {
            self.queue.write().await.insert(entry, DEFAULT_PRIORITY);
        }
    }

    async fn queue_as_vec(&self) -> Vec<PathBuf> {
        let mut result = Vec::<PathBuf>::new();
        while let Some(item) = self.queue.write().await.pop() {
            result.push(item.clone());
            self.queue.write().await.insert(item, DEFAULT_PRIORITY);
        }
        result
    }

    async fn contains(&self, directory: &PathBuf) -> bool {
        while let Some(item) = self.queue.write().await.pop() {
            let item_clone = item.clone();
            self.queue.write().await.insert(item, DEFAULT_PRIORITY);
            if &item_clone == directory {
                return true;
            }
        }
        false
    }
}
