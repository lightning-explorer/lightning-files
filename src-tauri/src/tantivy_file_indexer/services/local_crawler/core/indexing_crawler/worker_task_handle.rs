use tokio::task::JoinHandle;

use super::task_manager::CrawlerManagerMessageSender;

pub struct CrawlerWorkerTaskHandle {
    pub sender: CrawlerManagerMessageSender,
    task: JoinHandle<()>,
}

impl CrawlerWorkerTaskHandle {
    pub fn new(sender: CrawlerManagerMessageSender, task: JoinHandle<()>) -> Self {
        Self {
            sender,
            task,
        }
    }
    pub async fn run_until_complete(self){
        self.task.await;
    }
    pub fn is_finished(&self)->bool{
        self.task.is_finished()
    }
}
