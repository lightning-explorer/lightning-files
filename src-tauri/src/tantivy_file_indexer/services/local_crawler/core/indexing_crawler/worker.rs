use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use rand::{Rng, SeedableRng};
use tokio::sync::Notify;

use crate::{
    shared::models::sys_file_model::SystemFileModel,
    tantivy_file_indexer::shared::{
        async_retry,
        indexing_crawler::{
            models::crawler_file::CrawlerFile,
            traits::{commit_pipeline::CrawlerCommitPipeline, crawler_queue_api::CrawlerQueueApi},
        },
    },
};

use super::crawler::{self, CrawlerError};

pub struct IndexingCrawlerWorker<C, P>
where
    C: CrawlerQueueApi,
    P: CrawlerCommitPipeline<InputModel = SystemFileModel>,
{
    crawler_queue: Arc<C>,
    pipeline: Arc<P>,
    notify: Arc<Notify>,
    batch_size: usize,
}

impl<C, P> IndexingCrawlerWorker<C, P>
where
    C: CrawlerQueueApi,
    P: CrawlerCommitPipeline<InputModel = SystemFileModel>,
{
    /// Note that `worker_task` must be called in order for the background operations to start
    ///
    /// `batch_size` is the number of files that will be processed before the indexer commits them in bulk
    pub fn new(
        crawler_queue: Arc<C>,
        pipeline: Arc<P>,
        notify: Arc<Notify>,
        batch_size: usize,
    ) -> Self {
        Self {
            crawler_queue,
            pipeline,
            notify,
            batch_size,
        }
    }

    pub async fn worker_task(&self) {
        let mut dtos_bank: Vec<(CrawlerFile, Vec<SystemFileModel>)> = Vec::new();
        let mut num_files_processed = 0;

        self.random_wait().await;
        loop {
            // Since not every directory will have a lot of files, save up a bunch of files and then commit all of them
            match self.staggered_fetch_next().await {
                Ok(file_option) => match file_option {
                    Some(file) => {
                        // not needed if the log isn't there
                        // let file_clone = file.clone();
                        // let time = Instant::now();

                        let dtos = self.handle_crawl(&file).await;
                        num_files_processed += dtos.len();
                        dtos_bank.push((file, dtos));

                        // // optional log
                        // println!(
                        //     "Crawler worker finished processing {}. Priority: {}. Num files processed: {}. Time: {:?}",
                        //     file_clone.path.to_string_lossy(),
                        //     file_clone.priority,
                        //     num_files_processed,
                        //     time.elapsed()
                        // );

                        if num_files_processed >= self.batch_size {
                            // Commit all and drain the bank of files
                            num_files_processed = 0;
                            dtos_bank = self.commit_dtos_bank(dtos_bank).await;
                        }
                    }
                    None if !dtos_bank.is_empty() => {
                        // There isn't another item in the queue, but the crawler is still hanging on to DTOs
                        num_files_processed = 0;
                        dtos_bank = self.commit_dtos_bank(dtos_bank).await;
                    }
                    None => {
                        println!("No tasks available. Waiting for notification...");
                        self.notify.notified().await;
                        self.random_wait().await;
                    }
                },
                Err(err) => {
                    eprintln!(
                    "File crawler task encountered an error trying to fetch item from queue: {}. Retrying.",
                    err
                );
                    self.random_wait().await;
                }
            }
        }
    }

    async fn handle_index(&self, dir: &CrawlerFile, files: Vec<SystemFileModel>) {
        let parent = SystemFileModel::new_shallow(dir.path.to_string_lossy().to_string());
        match async_retry::retry_with_backoff(
            || self.pipeline.upsert_many(&files, &parent),
            5,
            Duration::from_millis(1000),
        )
        .await
        {
            Ok(_) => {
                match self.pipeline.commit_all().await {
                    Ok(()) => {
                        // If all goes well, then the directory can be removed from the crawler queue
                        if let Err(err) = self.remove_from_crawler_queue(dir).await {
                            // If for some reason the directory can't be removed, it is no big deal, it just means
                            // that it will get indexed again
                            println!(
                                "Error trying to remove directory from crawler queue: {}",
                                err
                            );
                        }
                    }
                    Err(err) => {
                        println!("Error committing files: {}", err);
                    }
                }
            }
            Err(err) => {
                println!("Error indexing files: {}", err);
            }
        }
    }

    /// Returns all of the files that were found in the given directory
    async fn handle_crawl(&self, directory: &CrawlerFile) -> Vec<SystemFileModel>
    where
        C: CrawlerQueueApi,
    {
        match crawler::crawl(directory, Arc::clone(&self.crawler_queue)).await {
            Ok(dtos) => {
                return dtos;
            }
            Err(err) => match err {
                CrawlerError::PushToQueueError(err) => {
                    println!(
                        "Crawler could not push found directories to queue: {}. 
            The original directory will stay in the queue for re-indexing",
                        err
                    );
                }
                CrawlerError::ReadDirError(err) => {
                    println!(
                        "Crawler could not read directory: {}. Removing it from the queue",
                        err
                    );
                    // Something must be wrong with the directory, so go ahead and remove it from the queue early
                    if let Err(err) = self.remove_from_crawler_queue(directory).await {
                        println!(
                            "Error trying to remove directory from crawler queue: {}",
                            err
                        );
                    }
                }
            },
        }
        Vec::new()
    }

    async fn commit_dtos_bank(
        &self,
        mut dtos: Vec<(CrawlerFile, Vec<SystemFileModel>)>,
    ) -> Vec<(CrawlerFile, Vec<SystemFileModel>)> {
        println!("Crawler is committing dtos bank");
        for (dir, files) in dtos.drain(..) {
            //println!("Draining {}", dir.path.to_string_lossy());
            self.handle_index(&dir, files).await;
        }
        dtos
    }

    async fn remove_from_crawler_queue(&self, directory: &CrawlerFile) -> Result<(), String> {
        async_retry::retry_with_backoff(
            || self.crawler_queue.delete_one(directory.clone()),
            5,
            Duration::from_millis(1000),
        )
        .await
    }

    /// Attempt to fetch the next item from the crawler queue, applying backoff if failing
    async fn staggered_fetch_next(&self) -> Result<Option<CrawlerFile>, String> {
        async_retry::retry_with_backoff(
            || self.crawler_queue.fetch_next(),
            8,
            Duration::from_millis(200),
        )
        .await
    }

    /// Stall for 100-2000 ms
    async fn random_wait(&self) {
        // thread safe rng
        let mut rng = rand_chacha::ChaChaRng::from_entropy();
        tokio::time::sleep(Duration::from_millis(rng.gen_range(100..=2000))).await;
    }
}
