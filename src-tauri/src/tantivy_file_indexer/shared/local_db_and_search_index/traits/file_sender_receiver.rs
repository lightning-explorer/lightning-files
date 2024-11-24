use std::future::Future;

use tokio::sync::mpsc::error::SendError;

pub trait FileIndexerReceiver<T> {
    async fn recv(&mut self) -> Option<T>;
}

pub trait FileIndexerSender<T>: Send + Clone + Sync + 'static
where
    T: Send,
{
    fn send(&self, value: T) -> impl Future<Output = Result<(), SendError<T>>> + Send;
}
