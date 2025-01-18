use std::{cmp::Ordering, sync::Arc, time::Duration};

use print_err::print_err;
use tokio::sync::{mpsc, RwLock};

use crate::tantivy_file_indexer::services::{
    local_crawler::core::{
        crawler_queue::queue::CrawlerQueue, indexing_crawler::plugins::throttle::ThrottleAmount,
    },
    search_index::pipelines::tantivy_pipeline::TantivyPipeline,
};

use super::{factory, worker_task_handle::CrawlerWorkerTaskHandle};

/// The max number of crawlers that can be active at once
const MAX_NUM_CRAWLERS: u32 = 8;
type CrawlerFactory = factory::IndexingCrawlersFactory<CrawlerQueue, TantivyPipeline>;
/// A message from the crawler task manager
pub enum CrawlerMessage {
    Kill,
    Throttle,
}
pub type CrawlerManagerMessageReceiver = mpsc::Receiver<CrawlerMessage>;
pub type CrawlerManagerMessageSender = mpsc::Sender<CrawlerMessage>;

pub async fn build_managed(mut factory: CrawlerFactory) {
    let num_workers = 8;

    factory = factory.set_batch_size(512);
    let tasks = factory.build(num_workers).await;

    println!(
        "Crawler task manager has spawned {} file crawlers",
        num_workers
    );
    let factory = Arc::new(RwLock::new(factory));
    {
        let mut factory_lock = factory.write().await;
        factory_lock.set_throttle(ThrottleAmount::None);
    }

    manage_crawl_tasks(tasks, factory);
}
fn manage_crawl_tasks(
    mut crawl_task_handles: Vec<CrawlerWorkerTaskHandle>,
    factory: Arc<RwLock<CrawlerFactory>>,
) {
    let check_frequency = Duration::from_secs(30);
    // TODO: have the task manager actually do stuff
    tokio::spawn(async move {
        for i in 0..10 {
            tokio::time::sleep(check_frequency).await;
            crawl_task_handles = remove_dead_crawlers(crawl_task_handles);
            let num_active_crawlers = crawl_task_handles.len() as u32;
            let recommended_crawlers = compute_recommended_num_crawlers().await;
            println!(
                "Crawler Task Manager: There are {} active crawlers and {} are recommended",
                num_active_crawlers, recommended_crawlers
            );
            match num_active_crawlers.cmp(&recommended_crawlers) {
                Ordering::Less => {
                    // Add more crawlers
                    let needed_crawlers = recommended_crawlers - num_active_crawlers;
                    let factory_lock = factory.read().await;

                    let new_crawlers = factory_lock.build(needed_crawlers).await;
                    // Put the new crawlers in with the rest of them
                    crawl_task_handles.extend(new_crawlers);
                }
                Ordering::Equal => { /* No action is needed */ }
                Ordering::Greater => {
                    let crawlers_to_kill = num_active_crawlers - recommended_crawlers;
                    kill_crawlers(&crawl_task_handles, crawlers_to_kill).await;
                }
            }
            println!("Crawler cycle {}", i);
        }
        println!("Killing all crawlers");
        kill_crawlers(&crawl_task_handles, 99).await;
    });
}

/// Sends a message to some of the crawlers to gracefully terminate them
async fn kill_crawlers(crawlers: &[CrawlerWorkerTaskHandle], amt: u32) {
    for i in 0..amt {
        if let Some(crawler) = crawlers.get(i as usize) {
            print_err(
                "CrawlerTaskManager",
                crawler.sender.send(CrawlerMessage::Kill).await,
            );
        } else {
            break;
        }
    }
}

/// Returns all of the crawlers that are still running
fn remove_dead_crawlers(crawlers: Vec<CrawlerWorkerTaskHandle>) -> Vec<CrawlerWorkerTaskHandle> {
    let mut alive_crawlers = Vec::new();
    for crawler in crawlers.into_iter() {
        if !crawler.is_finished() {
            alive_crawlers.push(crawler);
        }
    }
    alive_crawlers
}

/// Determine the recommended number of crawlers that should be active based on current CPU usage
async fn compute_recommended_num_crawlers() -> u32 {
    let cpu_usage = system_info::cpu::get_global_cpu_usage().await;
    let num = ((MAX_NUM_CRAWLERS * 8) as f32) / cpu_usage;
    let num = (num as u32).clamp(1, MAX_NUM_CRAWLERS);
    println!(
        "Crawler Task Manager: {} crawlers are recommended due to {}% CPU usage",
        num, cpu_usage
    );
    num
}
