use std::path::Path;

use crate::tantivy_file_indexer::{
    services::{
        local_db::tables::indexer_queue::api::IndexerQueueTable,
        search_index::models::index_worker::file_input::FileInputModel,
    },
    shared::local_db_and_search_index::traits::file_sender_receiver::FileIndexerReceiver,
};

pub struct DbConnectedReceiver {
    indexer_table: IndexerQueueTable,
}

impl DbConnectedReceiver {
    pub fn new(indexer_table: IndexerQueueTable) -> Self {
        Self { indexer_table }
    }
}

impl FileIndexerReceiver for DbConnectedReceiver {
    async fn recv(&mut self) -> Option<FileInputModel> {
        match self.indexer_table.pop().await {
            Ok(val) => {
                if let Some(m) = val {
                    return Some(FileInputModel {
                        directory_from: Path::new(&m.directory_from).to_path_buf(),
                        dtos: m.get_files(),
                    });
                }
                None
            }
            Err(err) => {
                println!("DbConnectedReceiver: Error during pop operation: {}", err);
                None
            }
        }
    }
}
