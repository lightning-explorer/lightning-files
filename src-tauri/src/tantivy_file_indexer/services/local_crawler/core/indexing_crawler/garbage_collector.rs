use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use futures::TryFutureExt;
use tokio::task::JoinHandle;

use crate::tantivy_file_indexer::{
    services::{
        app_save::service::AppSaveService, local_db::service::LocalDbService,
        search_index::service::SearchIndexService,
    },
    shared::async_retry,
};

pub struct CrawlerGarbageCollector {
    db_service: Arc<LocalDbService>,
    save_service: Arc<AppSaveService>,
    search_service: Arc<SearchIndexService>,

    num_files_processed: AtomicUsize,
    /// How big `num_files_processed` can get before it is persisted to disk
    mini_batch_size: usize,
    /// How big the total files processed counter can get before the garbage collection is invoked
    batch_size: usize,
}

impl CrawlerGarbageCollector {
    pub fn new(
        db_service: Arc<LocalDbService>,
        save_service: Arc<AppSaveService>,
        search_service: Arc<SearchIndexService>,
    ) -> Self {
        // Go ahead and create the file
        save_service
            .save("crawlers_n", 0)
            .expect("Unable to create file to save garbage collector data");
        Self {
            db_service,
            save_service,
            search_service,
            num_files_processed: AtomicUsize::new(0),
            mini_batch_size: 1000,
            batch_size: 30_000,
        }
    }

    pub fn register_num_files_processed(
        self: &Arc<Self>,
        num: usize,
    ) -> Result<(), std::io::Error> {
        self.num_files_processed.fetch_add(num, Ordering::Relaxed);
        let new_val = self.num_files_processed.load(Ordering::Relaxed);
        if new_val > self.mini_batch_size {
            let counter_path = "crawlers_n";
            // Reset counter of total files processed
            self.num_files_processed.store(0, Ordering::Relaxed);
            // Fetch the persisted counter from disk
            let disk_data = self.save_service.load::<usize>(counter_path)?;

            if disk_data > self.batch_size {
                // Write back to disk
                self.save_service.save(counter_path, 0)?;
                // Collect the garbage
                let self_clone = Arc::clone(self);
                self_clone.dispatch_garbage_collection();
            } else {
                self.save_service.save(counter_path, disk_data + new_val)?;
            }
        }
        Ok(())
    }

    fn dispatch_garbage_collection(self: Arc<Self>) -> JoinHandle<Result<(), String>> {
        tokio::task::spawn(async move { self.collect_garbage_task().await })
    }

    async fn collect_garbage_task(&self) -> Result<(), String> {
        // Attempt to vacuum the db
        println!("CrawlerGarbageCollector: Collecting garbage");
        async_retry::retry_with_backoff(
            |_| self.db_service.vacuum_database(),
            3,
            Duration::from_millis(1000),
        )
        .await
        .map_err(|err| err.to_string())?;

        // Attempt to merge segments
        self.search_service
            .merge_segments(10)
            .map_err(|err| err.to_string())
            .await?;

        Ok(())
    }
}
