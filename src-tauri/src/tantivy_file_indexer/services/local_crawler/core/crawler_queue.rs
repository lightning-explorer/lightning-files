use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use chrono::Utc;

use crate::tantivy_file_indexer::services::local_db::{
    service::LocalDbService,
    tables::{
        crawler_queue::entities::indexed_dir, recently_indexed_dirs::entities::recently_indexed_dir,
    },
};

pub type Priority = u32;
pub const DEFAULT_PRIORITY: Priority = 5;
pub struct CrawlerQueue {
    db: Arc<LocalDbService>,
}

// Rather than locally writing to JSON, write to the database.

impl CrawlerQueue {
    pub fn new(db: Arc<LocalDbService>) -> Self {
        Self { db }
    }

    pub async fn push_defaults(&self, paths: &[PathBuf]) {
        let files: Vec<(PathBuf, u32)> = paths
            .into_iter()
            .map(|path| (path.clone(), DEFAULT_PRIORITY))
            .collect();

        self.push_many(&files).await;
    }

    pub async fn pop(&self) -> Option<(PathBuf, Priority)> {
        println!("Length of queue: {}", self.get_len().await);

        match self.db.crawler_queue_table().pop().await {
            Ok(model) => model.map(|x| {
                if x.priority > 1 {
                    println!(
                        "Took item {} from queue with priority {}",
                        x.path, x.priority
                    );
                }
                (Path::new(&x.path).to_path_buf(), x.priority)
            }),
            Err(_) => None,
        }
    }

    pub async fn get_len(&self) -> u64 {
        self.db
            .crawler_queue_table()
            .count_dirs()
            .await
            .unwrap_or_default()
    }

    pub async fn push(&self, dir_path: PathBuf, priority: Priority) {
        self.push_many(&vec![(dir_path, priority)]).await;
    }

    pub async fn push_many(&self, entries: &[(PathBuf, u32)]) {
        // Remove the old directories to ensure that they can be indexed again
        // cutoff time is a value in minutes
        match &self.db.recently_indexed_dirs_table().refresh(5).await {
            Ok(val) => {
                if val > &0 {
                    println!("Found {} old directories in recently indexed and removed them to allow re-indexing", val);
                }
            }
            Err(err) => println!(
                "Failed to refresh recently indexed directories table: {}",
                err
            ),
        }

        // Filter out the entries that were indexed recently
        let entries = self.filter_out_recent_directories(entries).await;

        // Optional logging:
        if entries.is_empty() {
            println!("Crawler Queue: All directories that were attempted to be added have already been indexed recently");
        }

        let indexed_dir_models = self.entries_to_indexed_dir_model(&entries);
        // Add to the crawler queue
        if let Err(err) = self
            .db
            .crawler_queue_table()
            .upsert_many(&indexed_dir_models)
            .await
        {
            println!("Error pushing directories into queue: {}", err);
        }

        let recently_indexed_dir_models = self.entries_to_recently_indexed_model(&entries);
        // Add to recently indexed
        if let Err(err) = self
            .db
            .recently_indexed_dirs_table()
            .upsert_many(&recently_indexed_dir_models)
            .await
        {
            println!("Error adding directories to recently indexed: {}", err);
        }
    }

    async fn filter_out_recent_directories(
        &self,
        entries: &[(PathBuf, u32)],
    ) -> Vec<(PathBuf, u32)> {
        let mut res: Vec<(PathBuf, u32)> = Vec::new();
        for (path, priority) in entries.into_iter() {
            let is_recent = self
                .db
                .recently_indexed_dirs_table()
                .contains_dir(path.to_string_lossy().into_owned())
                .await
                .expect("Failed to check if directory was indexed recently");
            if !is_recent {
                res.push((path.clone(), *priority));
            }
        }
        res
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
}
