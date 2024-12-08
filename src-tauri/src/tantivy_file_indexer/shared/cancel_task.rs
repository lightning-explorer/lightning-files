use std::sync::Arc;
use tokio::sync::{oneshot, watch, RwLock};
use tokio::task::JoinHandle;

pub struct CancellableTask {
    current_task: Arc<RwLock<Option<(watch::Sender<()>, JoinHandle<()>)>>>,
}

impl CancellableTask {
    pub fn new() -> Self {
        Self {
            current_task: Arc::new(RwLock::new(None)),
        }
    }

    /// Runs a new task, canceling any previously running task.
    ///
    /// ### Arguments
    /// - `task`: A handle to the task
    ///
    /// ### Returns
    /// - `Ok(())` if the task completes successfully.
    /// - `Err(String)` if the task is canceled or fails.
    pub async fn run(&self, task: JoinHandle<()>) -> Result<(), String> {
        self.cancel_existing_task().await;

        let (cancel_sender, cancel_receiver) = watch::channel(());
        let (completion_sender, completion_receiver) = oneshot::channel();

        let handle = tokio::spawn(Self::task_runner(task, cancel_receiver, completion_sender));

        self.current_task
            .write()
            .await
            .replace((cancel_sender, handle));

        match completion_receiver.await {
            Ok(Ok(())) => Ok(()),
            Ok(Err(err)) => Err(err.to_string()),
            Err(_) => Err("Task completion channel was dropped".into()),
        }
    }

    /// Cancels any currently running task.
    async fn cancel_existing_task(&self) {
        if let Some((cancel_sender, old_handle)) = self.current_task.write().await.take() {
            let _ = cancel_sender.send(()); // Signal cancellation
            let _ = old_handle.await; // Wait for the task to finish
        }
    }

    /// Runs the provided task and handles cancellation.
    async fn task_runner(
        task: JoinHandle<()>,
        mut cancel_receiver: watch::Receiver<()>,
        completion_sender: oneshot::Sender<Result<(), &'static str>>,
    ) {
        let result = tokio::select! {
            _ = cancel_receiver.changed() => Err("Task was canceled."),
            _ = task => Ok(()),
        };

        let _ = completion_sender.send(result);
    }
}
