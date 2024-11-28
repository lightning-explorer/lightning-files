use crate::tantivy_file_indexer::{
    models::{search_params_model::SearchParamsModel, tantivy_file_model::TantivyFileModel},
    services::{local_db::service::LocalDbService, vector_db::service::VectorDbService},
    shared::local_db_and_search_index::db_connected_channel::{self, sender::DbConnectedSender},
};

use super::{
    core::{index_worker, querier, tantivy_setup},
    models::index_worker::file_input::FileInputModel,
};
use std::{path::PathBuf, sync::Arc};
use tantivy::{schema::Schema, IndexReader, IndexWriter};
use tauri::async_runtime::Sender;
use tokio::sync::{mpsc, Mutex};

pub struct SearchIndexService {
    pub schema: Schema,
    pub index_writer: Arc<Mutex<IndexWriter>>,
    index_reader: IndexReader,
    vector_db_service: Arc<VectorDbService>,
}

impl SearchIndexService {
    pub fn new(
        buffer_size: usize,
        app_path: PathBuf,
        vector_db_service: Arc<VectorDbService>,
    ) -> Self {
        let index_path = app_path.join("TantivyOut");

        let (schema, index_reader, index_writer) =
            tantivy_setup::initialize_tantity(buffer_size, index_path);

        Self {
            schema,
            index_writer: Arc::new(Mutex::new(index_writer)),
            index_reader,
            vector_db_service,
        }
    }

    pub fn query(
        &self,
        params: &SearchParamsModel,
    ) -> Result<Vec<TantivyFileModel>, tantivy::TantivyError> {
        querier::advanced_query(&self.schema, &self.index_reader.searcher(), params)
    }

    /**
    * Returns a `Sender` that a crawler can use to send over files.

    * The `batch_size` indicates how many files are processed before the index writer make a commit
    *
    * Note that right now, when the indexer is spawned, the vector indexer gets spawned as well
    */
    pub fn spawn_indexer_mpsc(
        &self,
        db_service: Arc<LocalDbService>,
        batch_size: usize,
        buffer_size: usize,
    ) -> Sender<FileInputModel> {
        // TODO: Look at buffer_size, because why is the Tokio channel and the vector indexer getting the same size?
        let schema_clone = Arc::new(self.schema.clone());
        let (sender, receiver) = mpsc::channel(buffer_size);

        let index_writer_clone = self.index_writer.clone();
        let vector_processor = Arc::new(
            self.vector_db_service
                .spawn_indexer(batch_size, buffer_size),
        );

        tokio::spawn(async move {
            index_worker::spawn_worker(
                receiver,
                index_writer_clone,
                schema_clone,
                db_service,
                vector_processor,
                batch_size,
            )
            .await;
        });

        sender
    }

    pub fn spawn_indexer_db_connected(
        &self,
        db_service: Arc<LocalDbService>,
        batch_size: usize,
        buffer_size: usize,
    ) -> DbConnectedSender {
        let schema_clone = Arc::new(self.schema.clone());
        let indexer_table_clone = db_service.indexer_queue_table().clone();
        let (sender, receiver) = db_connected_channel::channel::create(indexer_table_clone);

        let index_writer_clone = self.index_writer.clone();
        let vector_processor = Arc::new(
            self.vector_db_service
                .spawn_indexer(batch_size, buffer_size),
        );

        tokio::spawn(async move {
            index_worker::spawn_worker(
                receiver,
                index_writer_clone,
                schema_clone,
                db_service,
                vector_processor,
                batch_size,
            )
            .await;
        });

        sender
    }
}
