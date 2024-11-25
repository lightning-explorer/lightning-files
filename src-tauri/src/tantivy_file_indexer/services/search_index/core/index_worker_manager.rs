use std::{
    collections::HashSet,
    path::Path,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use crate::tantivy_file_indexer::{
    converters::date_converter::unix_time_to_tantivy_datetime,
    dtos::file_dto_input::FileDTOInput,
    services::{
        local_db::{
            service::LocalDbService,
            tables::files::{self},
        },
        vector_db::workers::indexer::VectorDbIndexer,
    },
    shared::local_db_and_search_index::traits::file_sender_receiver::FileIndexerReceiver,
};
use tantivy::{doc, schema::Schema, IndexWriter, TantivyError};
use tokio::sync::Mutex;

use super::index_worker::worker_task;

pub async fn spawn_workers<T>(
    receiver: T,
    writer: Arc<Mutex<IndexWriter>>,
    schema: Arc<Schema>,
    db_service: Arc<LocalDbService>,
    vector_db_indexer: Arc<VectorDbIndexer>,
    batch_size: usize,
    max_concurrent_tasks: usize,
) where
    T: FileIndexerReceiver,
{
    for id in 0..max_concurrent_tasks {
        tokio::spawn(worker_task(
            receiver.clone(),
            Arc::clone(&writer),
            Arc::clone(&schema),
            Arc::clone(&db_service),
            Arc::clone(&vector_db_indexer),
            batch_size
        ));
    }
}
