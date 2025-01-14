use rand::{Rng, SeedableRng};
use std::{sync::Arc, time::Duration};
use tokio::sync::mpsc::error::TryRecvError;

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

use super::{
    crawler::{self, CrawlerError},
    plugins::{filterer::CrawlerFilterer, garbage_collector::CrawlerGarbageCollector},
    task_manager::{CrawlerManagerMessageReceiver, CrawlerMessage},
};

pub struct IndexingCrawlerWorker<C, P>
where
    C: CrawlerQueueApi,
    P: CrawlerCommitPipeline<InputModel = SystemFileModel>,
{
    crawler_queue: Arc<C>,
    pipeline: Arc<P>,
    batch_size: usize,
    /// Message channel to receive orders from the task manager
    receiver: CrawlerManagerMessageReceiver,

    garbage_collector: Option<Arc<CrawlerGarbageCollector>>,
    filterer: Option<Arc<CrawlerFilterer>>,
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
        batch_size: usize,
        receiver: CrawlerManagerMessageReceiver,
    ) -> Self {
        Self {
            crawler_queue,
            pipeline,
            batch_size,
            receiver,

            garbage_collector: None,
            filterer: None,
        }
    }

    pub fn inject_garbage_collector(&mut self, c: Arc<CrawlerGarbageCollector>) {
        self.garbage_collector = Some(c);
    }

    pub fn inject_filterer(&mut self, f: Arc<CrawlerFilterer>) {
        self.filterer = Some(f);
    }

    pub async fn worker_task(&mut self) {
        let mut files_bank: Vec<(CrawlerFile, Vec<SystemFileModel>)> = Vec::new();
        let mut num_files_processed = 0;

        self.random_wait().await;
        loop {
            // Since not every directory will have a lot of files, save up a bunch of files and then commit all of them
            match self.staggered_fetch_next().await {
                Ok(file_option) => match file_option {
                    Some(file) => {
                        if let Some(filterer) = &self.filterer {
                            if !filterer.should_crawl_directory(&file.path).await {
                                println!("File Crawler - Filterer recommends not crawling directory: {}. Skipping it.",file.path.to_string_lossy());
                                // Fetched a directory that shouldn't be crawled:
                                // Example: 'node_modules'
                                self.remove_from_crawler_queue(&file).await;
                                continue;
                            }
                        }

                        let inner_files = self.handle_crawl(&file).await;
                        let len = inner_files.len();
                        num_files_processed += len;
                        // Register this number of files to the garbage collector, if there is one
                        if let Some(collector) = &self.garbage_collector {
                            if let Err(err) = collector.register_num_files_processed(len).await {
                                println!("Error registering files to gbg collector: {}", err);
                            }
                        }

                        files_bank.push((file, inner_files));

                        if num_files_processed >= self.batch_size {
                            // Commit all and drain the bank of files
                            num_files_processed = 0;
                            files_bank = self.commit_files_bank(files_bank).await;
                        }
                    }
                    None if !files_bank.is_empty() => {
                        // There isn't another item in the queue, but the crawler is still hanging on to files
                        num_files_processed = 0;
                        files_bank = self.commit_files_bank(files_bank).await;
                    }
                    None => {
                        println!("No tasks available. Waiting for notification...");
                        let queue_notifier = self.crawler_queue.get_notifier();
                        queue_notifier.notified().await;
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
            // Handle messages from the task manager:
            if let Some(message) = self.get_next_receiver_msg() {
                match message {
                    CrawlerMessage::Kill => break,
                    CrawlerMessage::Throttle(_) => todo!(),
                }
            }
        }
        println!("Crawler worker has been killed due to task manager saying so");
    }

    fn get_next_receiver_msg(&mut self) -> Option<CrawlerMessage> {
        match self.receiver.try_recv() {
            Ok(message) => return Some(message),
            Err(err) => {
                if let TryRecvError::Disconnected = err {
                    println!(
                        "WARNING: Disconnection between crawler and its task manager communication channel: {}",
                        err
                    )
                }
                // Otherwise, it'll be an `Empty` error, which we don't care about
            }
        }
        None
    }

    async fn handle_index(&self, dir: &CrawlerFile, files: Vec<SystemFileModel>) {
        let parent_path = dir.path.to_string_lossy().to_string();
        match async_retry::retry_with_backoff(
            |_| {
                self.pipeline
                    .upsert_many(files.clone(), parent_path.clone())
            },
            5,
            Duration::from_millis(1000),
        )
        .await
        {
            Ok(_) => {
                // If all goes well, then the directory can be removed from the crawler queue
                self.remove_from_crawler_queue(dir).await;
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
        let filterer_clone = self.filterer.clone();
        match crawler::crawl(directory, Arc::clone(&self.crawler_queue), filterer_clone).await {
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
                    self.remove_from_crawler_queue(directory).await
                }
            },
        }
        Vec::new()
    }

    async fn commit_files_bank(
        &self,
        mut files: Vec<(CrawlerFile, Vec<SystemFileModel>)>,
    ) -> Vec<(CrawlerFile, Vec<SystemFileModel>)> {
        //println!("Crawler is committing files bank");
        for (dir, inner_files) in files.drain(..) {
            //println!("Draining {}", dir.path.to_string_lossy());
            self.handle_index(&dir, inner_files).await;
        }
        files
    }

    async fn remove_from_crawler_queue(&self, directory: &CrawlerFile) {
        if let Err(err) = async_retry::retry_with_backoff(
            |_| self.crawler_queue.delete_one(directory.clone()),
            5,
            Duration::from_millis(1000),
        )
        .await
        {
            println!(
                "FileCrawlerWorker - Error removing directory from queue: {}",
                err
            );
        }
    }

    /// Attempt to fetch the next item from the crawler queue, applying backoff if failing
    async fn staggered_fetch_next(&self) -> Result<Option<CrawlerFile>, String> {
        async_retry::retry_with_backoff(
            |_| self.crawler_queue.fetch_next(),
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
