use std::sync::Arc;

use crate::tantivy_file_indexer::services::local_db::tables::indexer_queue::api::IndexerQueueTable;

use super::{receiver::DbConnectedReceiver, sender::DbConnectedSender};

pub fn create(indexer_table: Arc<IndexerQueueTable>) -> (DbConnectedSender, DbConnectedReceiver) {
    let sender = DbConnectedSender::new(Arc::clone(&indexer_table));
    let receiver = DbConnectedReceiver::new(Arc::clone(&indexer_table));
    (sender, receiver)
}
