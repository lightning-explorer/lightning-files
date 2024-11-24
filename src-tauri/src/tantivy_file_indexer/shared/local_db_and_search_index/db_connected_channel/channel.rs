use crate::tantivy_file_indexer::services::local_db::tables::indexer_queue::api::IndexerQueueTable;

use super::{receiver::DbConnectedReceiver, sender::DbConnectedSender};

pub fn create(indexer_table: IndexerQueueTable) -> (DbConnectedSender, DbConnectedReceiver) {
    let sender = DbConnectedSender::new(indexer_table.clone());
    let receiver = DbConnectedReceiver::new(indexer_table);
    (sender, receiver)
}
