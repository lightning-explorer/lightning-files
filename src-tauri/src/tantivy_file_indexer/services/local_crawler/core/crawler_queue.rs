use std::{path::PathBuf, sync::Arc};

use crossbeam::queue::SegQueue;

pub struct CrawlerQueue{
    queue:Arc<SegQueue<PathBuf>>
}

impl CrawlerQueue{
    pub fn new(directories:Vec<PathBuf>)->Self{
        let queue = Arc::new(SegQueue::<PathBuf>::new());
        for item in directories{
            queue.push(item);
        }
        Self{
            queue
        }
    }
    pub fn push(&self, directory:PathBuf){
        self.queue.push(directory);
    }
    pub fn pop(&self)->Option<PathBuf>{
        self.queue.pop()
    }
}