use std::future::Future;

use tokio::sync::mpsc::{self, error::SendError};

use crate::tantivy_file_indexer::shared::indexing_crawler::models::system_directory_model::InternalSystemDirectoryModel;

use super::super::file_sender_receiver::{FileIndexerReceiver, FileIndexerSender};

impl FileIndexerSender for mpsc::Sender<InternalSystemDirectoryModel> {
    fn send(
        &self,
        value: InternalSystemDirectoryModel,
    ) -> impl Future<Output = Result<(), SendError<InternalSystemDirectoryModel>>> + Send {
        Box::pin(async move { self.send(value).await })
    }
}

impl FileIndexerReceiver for tokio::sync::mpsc::Receiver<InternalSystemDirectoryModel> {
    async fn recv(&mut self) -> Option<InternalSystemDirectoryModel> {
        self.recv().await
    }
}
