use std::sync::Arc;

use tokio::sync::mpsc::error::SendError;

use crate::tantivy_file_indexer::{
    services::{
        local_db::tables::indexer_queue::api::IndexerQueueTable,
        search_index::models::index_worker::file_input::FileInputModel,
    },
    shared::local_db_and_search_index::traits::file_sender_receiver::FileIndexerSender,
};

#[derive(Clone)]
pub struct DbConnectedSender {
    indexer_table: Arc<IndexerQueueTable>,
}

impl DbConnectedSender{
    pub fn new(indexer_table: Arc<IndexerQueueTable>) -> Self {
        Self { indexer_table }
    }
}

impl FileIndexerSender for DbConnectedSender {
    fn send(
        &self,
        value: FileInputModel,
    ) -> impl std::future::Future<Output = Result<(), SendError<FileInputModel>>> + Send {
        let value_clone: FileInputModel = value.clone();
        let indexer_table_clone = Arc::clone(&self.indexer_table);
        Box::pin(async move {
            match indexer_table_clone.add(value).await {
                Ok(_) => Ok(()),
                Err(_) => Err(SendError(value_clone)),
            }
        })
    }
}
