use std::sync::Arc;
use tokio::sync::{oneshot, watch, RwLock};
use tokio::task::JoinHandle;

type AtomicOption<T> = Arc<RwLock<Option<T>>>;
pub struct CancellableTask {
    current_task: AtomicOption<(watch::Sender<()>, JoinHandle<()>)>,
}

impl CancellableTask {
    pub fn new() -> Self {
        Self {
            current_task: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn run<T>(&self, task: JoinHandle<T>) -> Result<T, String>
    where
        T: Send + 'static,
    {
        self.run_internal(task, || {}).await
    }
 
    pub async fn run_with_cleanup<T, F>(
        &self,
        task: JoinHandle<T>,
        on_cancel: F,
    ) -> Result<T, String>
    where
        T: Send + 'static,
        F: Fn() + Send + 'static,
    {
        self.run_internal(task, on_cancel).await
    }

    /// Runs a new task, canceling any previously running task.
    async fn run_internal<T, F>(&self, task: JoinHandle<T>, on_cancel: F) -> Result<T, String>
    where
        T: Send + 'static,
        F: Fn() + Send + 'static,
    {
        self.cancel_existing_task().await;

        let (cancel_tx, mut cancel_rx) = watch::channel(());
        let (result_tx, result_rx) = oneshot::channel();

        let handle = tokio::spawn(async move {
            let result = tokio::select! {
                _ = cancel_rx.changed() => {
                    on_cancel();
                    Err("Task was canceled.")
                },
                res = task => Ok(res),
            };
            let _ = result_tx.send(result);
        });

        self.current_task.write().await.replace((cancel_tx, handle));

        match result_rx.await {
            Ok(Ok(res)) => res.map_err(|e| e.to_string()),
            Ok(Err(e)) => Err(e.to_string()),
            Err(_) => Err("Task completion channel was dropped".into()),
        }
    }

    /// Cancels any currently running task.
    async fn cancel_existing_task(&self) {
        if let Some((cancel_tx, handle)) = self.current_task.write().await.take() {
            let _ = cancel_tx.send(()); // Signal cancellation
            let _ = handle.await; // Wait for the task to finish
        }
    }
}
