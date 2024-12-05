use std::future::Future;

use tokio::sync::mpsc::error::SendError;

use crate::tantivy_file_indexer::shared::indexing_crawler::models::system_directory_model::InternalSystemDirectoryModel;

pub trait FileIndexerReceiver {
    async fn recv(&mut self) -> Option<InternalSystemDirectoryModel>;
}

pub trait FileIndexerSender: Send + Clone + Sync + 'static {
    fn send(
        &self,
        value: InternalSystemDirectoryModel,
    ) -> impl Future<Output = Result<(), SendError<InternalSystemDirectoryModel>>> + Send;
}
//merge
