use std::future::Future;

use tokio::sync::mpsc::{self, error::SendError};

use crate::tantivy_file_indexer::services::search_index::models::index_worker::file_input::FileInputModel;

use super::super::file_sender_receiver::{FileIndexerReceiver, FileIndexerSender};

impl FileIndexerSender for mpsc::Sender<FileInputModel>
{
    fn send(&self, value: FileInputModel) -> impl Future<Output = Result<(), SendError<FileInputModel>>> + Send {
        Box::pin(async move { self.send(value).await })
    }
}

impl FileIndexerReceiver for tokio::sync::mpsc::Receiver<FileInputModel> {
    async fn recv(&mut self) -> Option<FileInputModel> {
        self.recv().await
    }
}
