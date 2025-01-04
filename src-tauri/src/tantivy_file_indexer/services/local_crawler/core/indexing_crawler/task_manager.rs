use tokio::{sync::Mutex, task::{JoinHandle, JoinSet}};

use crate::tantivy_file_indexer::services::{local_crawler::core::crawler_queue::queue::CrawlerQueue, search_index::pipelines::tantivy_pipeline::TantivyPipeline};

use super::builder;

pub async fn build_managed(builder:builder::IndexingCrawlersBuilder<CrawlerQueue, TantivyPipeline>){
    let tasks = builder
    .max_num_workers(2)
    .batch_size(512)
    .build_async().await;
    
    manage_crawl_tasks(tasks);
}
async fn manage_crawl_tasks(crawl_task_handles:JoinSet<()>){

    tokio::spawn(async move{
        crawl_task_handles
    })
}