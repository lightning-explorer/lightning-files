use std::{sync::Arc, time::Instant};

use tokio::sync::{mpsc::error::SendError, Notify};

use crate::tantivy_file_indexer::{
    services::{
        local_db::tables::indexer_queue::api::IndexerQueueTable,
        search_index::models::index_worker::file_input::FileInputModel,
    },
    shared::local_db_and_search_index::traits::file_sender_receiver::FileIndexerSender,
};

#[derive(Clone)]
pub struct DbConnectedSender {
    indexer_table: IndexerQueueTable,
    notify: Arc<Notify>,
}

impl DbConnectedSender {
    pub fn new(indexer_table: IndexerQueueTable) -> Self {
        Self {
            indexer_table,
            notify: Arc::new(Notify::new()),
        }
    }
    /**
    Because the sender triggers a notification every time a value gets sent, you can subscribe to the notifications
    */
    pub fn get_notify(&self) -> Arc<Notify> {
        Arc::clone(&self.notify)
    }
}

impl FileIndexerSender for DbConnectedSender {
    fn send(
        &self,
        value: FileInputModel,
    ) -> impl std::future::Future<Output = Result<(), SendError<FileInputModel>>> + Send {
        let value_clone: FileInputModel = value.clone();
        let indexer_table_clone = Arc::new(&self.indexer_table);
        let time = Instant::now();

        // Send a notification
        self.notify.notify_one();

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
