use std::sync::Arc;

use crate::tantivy_file_indexer::{
    services::{local_db::service::LocalDbService, vector_db::workers::indexer::VectorDbIndexer},
    shared::local_db_and_search_index::traits::file_sender_receiver::FileIndexerReceiver,
};
use tantivy::{schema::Schema, IndexWriter};
use tokio::{sync::Mutex, task::JoinSet};

use super::index_worker::worker_task;

pub fn spawn_workers<T>(
    receiver: T,
    writer: Arc<Mutex<IndexWriter>>,
    schema: Arc<Schema>,
    db_service: Arc<LocalDbService>,
    vector_db_indexer: Arc<VectorDbIndexer>,
    batch_size: usize,
    max_concurrent_tasks: usize,
) -> JoinSet<()>
where
    T: FileIndexerReceiver,
{
    let mut tasks = JoinSet::new();

    for _ in 0..max_concurrent_tasks {
        tasks.spawn(worker_task(
            receiver.clone(),
            Arc::clone(&writer),
            Arc::clone(&schema),
            Arc::clone(&db_service),
            Arc::clone(&vector_db_indexer),
            batch_size,
        ));
    }

    tasks
}
