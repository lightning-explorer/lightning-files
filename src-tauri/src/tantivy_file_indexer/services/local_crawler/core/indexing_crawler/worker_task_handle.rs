use tokio::task::JoinHandle;

use super::task_manager::CrawlerManagerMessageSender;

pub struct CrawlerWorkerTaskHandle {
    sender: Option<CrawlerManagerMessageSender>,
    task: JoinHandle<()>,
}

impl CrawlerWorkerTaskHandle {
    pub fn new(sender: CrawlerManagerMessageSender, task: JoinHandle<()>) -> Self {
        Self {
            sender: Some(sender),
            task,
        }
    }
    pub fn take_sender(&mut self) -> CrawlerManagerMessageSender {
        self.sender
            .take()
            .expect("The CrawlerManagerMessageSender has already been taken from this crawler")
    }
    pub fn take_handle(self) -> JoinHandle<()> {
        self.task
    }
}
