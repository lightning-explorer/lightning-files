use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use tantivy::{doc, TantivyError};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

use crate::tantivy_file_indexer::{
    app_data,
    converters::date_converter::unix_time_to_tantivy_datetime,
    db::{sqlx_service::SqlxService, tables::files::models::FileModel},
    dtos::file_dto_input::FileDTOInput,
    service::search_index_service::SearchIndexService,
    service_container::AppServiceContainer,
};

use super::walker::{self, FileWalker};

const WALKER_QUEUE_NAME: &str = "walker_queue";
pub struct Crawler {
    search_service: Arc<SearchIndexService>,
    db_service: Arc<SqlxService>,
}

impl Crawler {
    pub fn new_from_service(service: &AppServiceContainer) -> Arc<Self> {
        let this = Self {
            search_service: service.search_service.clone(),
            db_service: service.sqlx_service.clone(),
        };
        Arc::new(this)
    }

    pub async fn crawl_existing(
        self: Arc<Self>,
        fallback_directory: &str,
        batch_size: usize,
        max_concurrent_tasks: usize,
    ) {
        let existing_paths = self.get_walker_existing_paths();
        if existing_paths.len() < 1 {
            self.crawl_new(fallback_directory, batch_size, max_concurrent_tasks)
                .await;
        } else {
            let mut walker = walker::FileWalker::new_from_queue(existing_paths);
            self.crawl(&mut walker, batch_size, max_concurrent_tasks)
                .await;
        }
    }

    async fn crawl_new(
        self: Arc<Self>,
        directory: &str,
        batch_size: usize,
        max_concurrent_tasks: usize,
    ) {
        let mut walker = walker::FileWalker::new(&directory).expect("Can't walk directory");
        self.crawl(&mut walker, batch_size, max_concurrent_tasks)
            .await;
    }

    async fn crawl(
        self: Arc<Self>,
        walker: &mut FileWalker,
        batch_size: usize,
        max_concurrent_tasks: usize,
    ) {
        let mut batches_dispatched: u32 = 0;
        let semaphore = Arc::new(Semaphore::new(max_concurrent_tasks));

        let mut tasks = Vec::new();

        // each call to 'next' will return every file/directory path as a DTO
        while let Some((dir_path, dtos)) = walker.next() {
            let seen_paths: HashSet<String> = dtos.iter().map(|x| x.file_path.clone()).collect();

            for dto in dtos.into_iter() {
                let permit = semaphore.clone().acquire_owned().await.unwrap();
                let task = self.clone().process_file(dto, permit).await;
                tasks.push(task);

                // Commit and reset tasks when batch_size is reached
                if tasks.len() >= batch_size {
                    if let Err(err) = self.save_walker_queue(walker) {
                        println!("Couldn't save walker queue: {}", err);
                    }
                    self.process_batch(&mut tasks).await;

                    // Optional logging
                    batches_dispatched += 1;
                    println!(
                        "dispatched {} files",
                        batches_dispatched * (batch_size as u32)
                    );
                }
            }

            match self.remove_unseen_entries(dir_path, seen_paths).await {
                Ok(val) => {
                    if val > 0 {
                        println!("Removed {} stale entries", val);
                    }
                }
                Err(err) => {
                    println!("Couldn't remove unseen entries: {}", err);
                }
            }
        }
    }

    async fn process_file(
        self: Arc<Self>,
        dto: FileDTOInput,
        permit: OwnedSemaphorePermit,
    ) -> tokio::task::JoinHandle<()> {
        let index_writer_clone = self.search_service.index_writer.clone();
        let schema = self.search_service.schema.clone();
        let db_service = self.db_service.clone();
        let self_clone = self.clone();

        tokio::task::spawn(async move {
            let _permit = permit; // Ensure semaphore is released at end
            if let Err(err) = self_clone.remove_file_from_index(&dto.file_path).await {
                eprintln!("Error removing file from index: {}", err);
            }

            let writer = index_writer_clone.lock().await;
            if let Err(err) = writer.add_document(doc! {
                schema.get_field("file_id").unwrap() => dto.file_id,
                schema.get_field("name").unwrap() => dto.name,
                schema.get_field("date_modified").unwrap() => unix_time_to_tantivy_datetime(dto.date_modified),
                schema.get_field("path").unwrap() => dto.file_path.clone(),
                schema.get_field("metadata").unwrap() => dto.metadata,
                schema.get_field("popularity").unwrap() => dto.popularity,
            }) {
                eprintln!("Error adding document to index: {}", err);
            }

            let path_clone = dto.file_path.clone();
            let parent_path = self.get_parent_path(path_clone);

            let file_model = FileModel {
                path: dto.file_path,
                parent_path,
            };
            if let Err(err) = db_service.files_table.upsert(&file_model).await {
                eprintln!("Error upserting file model: {}", err);
            }
        })
    }

    async fn process_batch(&self, tasks: &mut Vec<tokio::task::JoinHandle<()>>) {
        for task in tasks.drain(..) {
            task.await.unwrap();
        }

        if let Err(err) = self.commit_and_retry().await {
            eprintln!("Error committing batch: {}", err);
        }
    }

    async fn remove_unseen_entries(
        &self,
        directory: PathBuf,
        seen_paths: HashSet<String>,
    ) -> Result<usize, String> {
        let stored_paths = self
            .db_service
            .files_table
            .get_paths_from_dir(&directory.to_string_lossy())
            .await
            .map_err(|e| e.to_string())?;

        let stale_paths: HashSet<_> = stored_paths.difference(&seen_paths).cloned().collect();
        let stale_paths_len = stale_paths.len();

        for path in stale_paths {
            if let Err(err) = self.remove_file_from_index(&path).await {
                return Err(err.to_string());
            }
            if let Err(err) = self.db_service.files_table.remove_path(&path).await {
                return Err(err.to_string());
            }
        }
        Ok(stale_paths_len)
    }

    async fn commit_and_retry(&self) -> Result<(), TantivyError> {
        let retry_attempts = 3;

        for attempt in 1..=retry_attempts {
            match self.search_service.index_writer.lock().await.commit() {
                Ok(_) => return Ok(()),
                Err(e) if attempt < retry_attempts => {
                    eprintln!("Commit failed (attempt {}), retrying: {:?}", attempt, e);
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    async fn remove_file_from_index(&self, file_path: &str) -> tantivy::Result<()> {
        let index_writer = self.search_service.index_writer.lock().await;
        let field = self.search_service.schema.get_field("file_id")?;
        index_writer.delete_term(tantivy::Term::from_field_text(field, file_path));
        Ok(())
    }

    fn get_parent_path(&self, path: String) -> Option<String> {
        let path = Path::new(&path);
        match path.parent() {
            Some(val) => Some(val.to_string_lossy().to_string()),
            None => None,
        }
    }

    fn save_walker_queue(&self, walker: &FileWalker) -> Result<(), std::io::Error> {
        let paths = walker.get_current_queue();
        app_data::json::save(WALKER_QUEUE_NAME, paths)
    }

    fn get_walker_existing_paths(&self) -> Vec<String> {
        app_data::json::load::<Vec<String>>(WALKER_QUEUE_NAME).unwrap_or_else(|err| {
            println!("Walker error getting existing paths: {}", err);
            return Vec::new();
        })
    }
}
