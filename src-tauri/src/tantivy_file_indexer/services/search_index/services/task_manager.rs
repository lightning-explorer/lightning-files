use tokio::{
    sync::{watch, RwLock},
    task::JoinHandle,
};

pub struct TaskManagerService {
    pub current_task: RwLock<Option<(watch::Sender<()>, JoinHandle<()>)>>,
}

impl TaskManagerService {
    pub fn new() -> Self {
        Self {
            current_task: RwLock::new(None),
        }
    }
}
