use std::future::Future;

use tokio::sync::mpsc::{self, error::SendError};

use super::super::file_sender_receiver::{FileIndexerReceiver, FileIndexerSender};

impl<T> FileIndexerSender<T> for mpsc::Sender<T>
where
    T: Send + 'static,
{
    fn send(&self, value: T) -> impl Future<Output = Result<(), SendError<T>>> + Send {
        Box::pin(async move { self.send(value).await })
    }
}

impl<T> FileIndexerReceiver<T> for tokio::sync::mpsc::Receiver<T> {
    async fn recv(&mut self) -> Option<T> {
        self.recv().await
    }
}
