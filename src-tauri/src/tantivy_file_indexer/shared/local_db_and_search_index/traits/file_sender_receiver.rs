use std::future::Future;

use tokio::sync::mpsc::error::SendError;

use crate::tantivy_file_indexer::services::search_index::models::index_worker::file_input::FileInputModel;

pub trait FileIndexerReceiver: Send + Clone + Sync + 'static {
    fn recv(&mut self) -> impl Future<Output = Option<FileInputModel>> + Send;
}

pub trait FileIndexerSender: Send + Clone + Sync + 'static {
    fn send(
        &self,
        value: FileInputModel,
    ) -> impl Future<Output = Result<(), SendError<FileInputModel>>> + Send;
}
