use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

use chrono::Utc;
use sea_orm::DbErr;
use tokio::sync::Notify;

use crate::tantivy_file_indexer::services::local_db::{
    service::LocalDbService,
    tables::{
        crawler_queue::{api::CrawlerQueueTable, entities::indexed_dir},
        recently_indexed_dirs::{
            api::RecentlyIndexedDirectoriesTable, entities::recently_indexed_dir,
        },
    },
};

pub type Priority = u32;
pub const DEFAULT_PRIORITY: Priority = 5;

#[derive(Clone)]
pub struct CrawlerQueue {
    db: Arc<LocalDbService>,
    notify: Arc<Notify>,
    /// The UNIX timestamp representing when the database was last vacuumed
    last_vacuum: Arc<AtomicI64>,
}

// Rather than locally writing to JSON, write to the database.

impl CrawlerQueue {
    pub fn new(db: Arc<LocalDbService>) -> Self {
        Self {
            db,
            notify: Arc::new(Notify::new()),
            last_vacuum: Arc::new(AtomicI64::new(0)),
        }
    }

    pub async fn push_defaults(&self, paths: &[PathBuf]) -> Result<(), DbErr> {
        let files: Vec<(PathBuf, u32)> = paths
            .iter()
            .map(|path| (path.clone(), DEFAULT_PRIORITY))
            .collect();

        self.push_many(&files).await?;
        Ok(())
    }

    pub async fn fetch_many(&self, amount: u64) -> Result<Vec<(PathBuf, Priority)>, DbErr> {
        self.get_crawler_queue_table()
            .get_many(amount)
            .await
            .map(|models| {
                models
                    .into_iter()
                    .map(|model| (PathBuf::from(model.path), model.priority))
                    .collect()
            })
    }

    pub async fn take_many(&self, amount: u64) -> Result<Vec<(PathBuf, Priority)>, DbErr> {
        self.get_crawler_queue_table()
            .take_many(amount)
            .await
            .map(|models| {
                models
                    .into_iter()
                    .map(|model| (PathBuf::from(model.path), model.priority))
                    .collect()
            })
    }

    pub async fn delete_many(&self, models: Vec<indexed_dir::Model>) -> Result<(), DbErr> {
        self.get_crawler_queue_table()
            .delete_many(&models)
            .await
            .map(|_| ())
    }

    pub async fn set_taken_to_false_all(&self) -> Result<(), DbErr> {
        self.get_crawler_queue_table().mark_all_as_not_taken().await
    }

    pub async fn pop(&self) -> Result<Option<(PathBuf, Priority)>, DbErr> {
        #[cfg(feature = "file_crawler_logs")]
        println!("Length of queue: {}", self.get_len().await);

        match self.get_crawler_queue_table().pop().await {
            Ok(model) => Ok(model.map(|x| {
                if x.priority > 1 {
                    #[cfg(feature = "file_crawler_logs")]
                    println!(
                        "Took item {} from queue with priority {}",
                        x.path, x.priority
                    );
                }
                (PathBuf::from(&x.path), x.priority)
            })),
            Err(err) => Err(err),
        }
    }

    pub async fn get_len(&self) -> u64 {
        self.get_crawler_queue_table()
            .count_dirs()
            .await
            .unwrap_or_default()
    }

    /// This function automatically gates off files that have been indexed recently, meaning that the fetch functions do not need to worry
    /// about grabbing entries that just got indexed.
    pub async fn push_many(&self, entries: &[(PathBuf, u32)]) -> Result<(), DbErr> {
        // Remove the old directories to ensure that they can be indexed again
        // cutoff time is a value in minutes

        // Common error: This table often fails to refresh
        match &self.get_recently_indexed_dirs_table().refresh(5).await {
            Ok(val) => {
                if val > &0 {
                    #[cfg(feature = "file_crawler_logs")]
                    println!("Found {} old directories in recently indexed and removed them to allow re-indexing", val);
                }
            }
            Err(err) => println!(
                "Failed to refresh recently indexed directories table: {}",
                err
            ),
        }

        // Filter out the entries that were indexed recently
        let entries = self.filter_out_recent_directories(entries).await?;

        // Optional logging:
        if entries.is_empty() {
            #[cfg(feature = "file_crawler_logs")]
            println!("Crawler Queue: All directories that were attempted to be added have already been indexed recently");
        }

        let indexed_dir_models = self.entries_to_indexed_dir_model(&entries);

        // Add to the crawler queue
        self.get_crawler_queue_table()
            .upsert_many(&indexed_dir_models)
            .await
            .map_err(|err| {
                DbErr::Custom(format!(
                    "Error upserting directories to crawler queue: {}",
                    err
                ))
            })?;

        // Notify all workers
        self.notify.notify_waiters();

        let recently_indexed_dir_models = self.entries_to_recently_indexed_model(&entries);
        // Add to recently indexed
        self.get_recently_indexed_dirs_table()
            .upsert_many(&recently_indexed_dir_models)
            .await
            .map_err(|err| {
                DbErr::Custom(format!(
                    "Error upserting directories to crawler queue: {}",
                    err
                ))
            })?;

        Ok(())
    }

    async fn filter_out_recent_directories(
        &self,
        entries: &[(PathBuf, u32)],
    ) -> Result<Vec<(PathBuf, u32)>, DbErr> {
        let mut res: Vec<(PathBuf, u32)> = Vec::new();
        for (path, priority) in entries.iter() {
            let is_recent = self
                .get_recently_indexed_dirs_table()
                .contains_dir(path.to_string_lossy().into_owned())
                .await?;
            if !is_recent {
                res.push((path.clone(), *priority));
            }
        }
        Ok(res)
    }

    pub async fn vacuum_db(&self) -> Result<(), DbErr> {
        let now = Utc::now().timestamp();
        let last_vacuum_timestamp = self.last_vacuum.load(Ordering::SeqCst);
        let diff = (now - last_vacuum_timestamp).abs();
        // Check if the difference exceeds 10 minutes (600 seconds)
        // This helps avoid too many consecutive vacuum calls
        if diff > 600 {
            self.last_vacuum.store(now, Ordering::SeqCst);
            self.db.vacuum_database().await?;
        } // else, ignore vacuuming the db for now
        Ok(())
    }

    /**
     Turns the entries into a format accepted by the crawler queue
    */
    fn entries_to_indexed_dir_model(&self, entries: &[(PathBuf, u32)]) -> Vec<indexed_dir::Model> {
        entries
            .iter()
            .map(|(path, priority)| indexed_dir::Model {
                path: path.to_string_lossy().into_owned(),
                priority: *priority,
                taken: false, // TODO: ensure setting this to false is correct
            })
            .collect()
    }

    /**
     Turns the entries into a format accepted by the recently indexed directories
    */
    fn entries_to_recently_indexed_model(
        &self,
        entries: &[(PathBuf, u32)],
    ) -> Vec<recently_indexed_dir::Model> {
        entries
            .iter()
            .map(|(path, _)| recently_indexed_dir::Model {
                path: path.to_string_lossy().into_owned(),
                time: Utc::now().timestamp(),
            })
            .collect()
    }

    pub fn get_notifier(&self) -> Arc<Notify> {
        Arc::clone(&self.notify)
    }

    fn get_crawler_queue_table(&self) -> &CrawlerQueueTable {
        self.db.crawler_queue_table()
    }

    fn get_recently_indexed_dirs_table(&self) -> &RecentlyIndexedDirectoriesTable {
        self.db.recently_indexed_dirs_table()
    }
}
