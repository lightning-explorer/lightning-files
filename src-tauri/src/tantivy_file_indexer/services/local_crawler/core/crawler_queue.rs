use std::{path::PathBuf, sync::Arc};

use crossbeam::queue::SegQueue;
use tokio::sync::Mutex;

use crate::tantivy_file_indexer::app_data;

const SAVE_PATH: &str = "files_queue";
pub struct CrawlerQueue {
    queue: Arc<SegQueue<PathBuf>>,
    iter_on: Arc<Mutex<usize>>,
    save_after_iters: usize,
}

impl CrawlerQueue {
    /**
     * An 'iteration' is one file being processed. So `save_after_iters` should be a pretty sizeable value
     */
    pub fn new(directories: Vec<PathBuf>, save_after_iters: usize) -> Self {
        let queue = Arc::new(SegQueue::<PathBuf>::new());
        for item in directories {
            queue.push(item);
        }
        Self {
            queue,
            iter_on: Arc::new(Mutex::new(0)),
            save_after_iters,
        }
    }

    pub fn push(&self, directory: PathBuf) {
        self.queue.push(directory);
    }

    pub async fn pop(&self) -> Option<PathBuf> {
        self.increment_iter().await;
        self.queue.pop()
    }

    pub fn len(&self)->usize{
        self.queue.len()
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        app_data::json::save(SAVE_PATH, self.queue_as_vec())
    }

    /**
     * Fallback in case the save path doesn't exist
     */
    pub fn load_or(&self, fallback_directories: Vec<PathBuf>) {
        if self.load().is_err() {
            self.populate_queue(fallback_directories);
        }
    }

    pub fn load(&self) -> Result<(), std::io::Error> {
        let entries = app_data::json::load::<Vec<PathBuf>>(SAVE_PATH)?;
        self.populate_queue(entries);
        Ok(())
    }

    async fn increment_iter(&self) {
        let mut iter_on = self.iter_on.lock().await;
        if *iter_on >= self.save_after_iters {
            *iter_on = 0;
            if let Err(err) = self.save() {
                println!("Failed to save crawler queue: {}", err);
            }
        } else {
            *iter_on += 1;
        }
    }

    fn populate_queue(&self, entries: Vec<PathBuf>) {
        for entry in entries.into_iter() {
            self.queue.push(entry);
        }
    }

    fn queue_as_vec(&self) -> Vec<PathBuf> {
        let mut result = Vec::<PathBuf>::new();
        while let Some(item) = self.queue.pop() {
            result.push(item.clone());
            self.queue.push(item);
        }
        result
    }
}
