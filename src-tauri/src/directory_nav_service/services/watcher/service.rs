use std::{path::PathBuf, sync::Arc};

use crate::tantivy_file_indexer::shared::cancel_task::CancellableTask;

use super::notifier;

pub struct DirectoryWatcherService {
    watcher_task: CancellableTask,
}

impl DirectoryWatcherService {
    pub fn new() -> Self {
        Self {
            watcher_task: CancellableTask::new(),
        }
    }
    pub fn watch<F>(self:&Arc<Self>, dir: PathBuf, on_changes: F)
    where
        F: Fn() + Send + 'static,
    {
        let task = tokio::task::spawn(async move { notifier::watcher_task(&dir, on_changes) });
        let self_clone = Arc::clone(self);
        tokio::task::spawn(async move {
            if let Err(err) = self_clone.watcher_task.run(task).await{
                println!("Directory watcher task error: {}",err);
            }
        });
    }
    pub async fn stop_watching(&self){
        self.watcher_task.cancel().await;
    }
}
