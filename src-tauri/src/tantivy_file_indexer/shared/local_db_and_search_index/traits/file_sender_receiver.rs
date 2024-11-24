use std::future::Future;

use tokio::sync::mpsc::error::SendError;

use crate::tantivy_file_indexer::services::search_index::models::index_worker::file_input::FileInputModel;

pub trait FileIndexerReceiver {
    async fn recv(&mut self) -> Option<FileInputModel>;
}

pub trait FileIndexerSender: Send + Clone + Sync + 'static
{
    fn send(&self, value: FileInputModel) -> impl Future<Output = Result<(), SendError<FileInputModel>>> + Send;
}
