use std::{sync::Arc, time::Instant};

use tokio::sync::mpsc::error::SendError;

use crate::tantivy_file_indexer::{
    services::local_db::tables::indexer_queue::api::IndexerQueueTable,
    shared::{
        indexing_crawler::models::system_directory_model::InternalSystemDirectoryModel,
        local_db_and_search_index::traits::file_sender_receiver::FileIndexerSender,
    },
};

#[derive(Clone)]
pub struct DbConnectedSender {
    indexer_table: IndexerQueueTable,
}

impl DbConnectedSender {
    pub fn new(indexer_table: IndexerQueueTable) -> Self {
        Self { indexer_table }
    }
}

impl FileIndexerSender for DbConnectedSender {
    fn send(
        &self,
        value: InternalSystemDirectoryModel,
    ) -> impl std::future::Future<Output = Result<(), SendError<InternalSystemDirectoryModel>>> + Send
    {
        let value_clone: InternalSystemDirectoryModel = value.clone();
        let indexer_table_clone = Arc::new(&self.indexer_table);
        let time = Instant::now();
        Box::pin(async move {
            match indexer_table_clone.add(value).await {
                Ok(_) => {
                    #[cfg(feature = "db_indexer_queue_logs")]
                    println!(
                        "FileIndexerSender send + JSON serialization took {:?}",
                        time.elapsed()
                    );

                    Ok(())
                }
                Err(_) => Err(SendError(value_clone)),
            }
        })
    }
}
