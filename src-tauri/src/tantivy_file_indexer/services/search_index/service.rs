use crate::{
    shared::dtos::file_dto::FileDTO,
    tantivy_file_indexer::{
        models::search_params_model::SearchParamsModel,
        services::{local_db::service::LocalDbService, vector_db::service::VectorDbService},
        shared::local_db_and_search_index::db_connected_channel::{
            self, sender::DbConnectedSender,
        },
    },
};

use super::{
    super::super::{configs::file_indexer_config::FileIndexerConfig, schemas::file_schema},
    core::{index_worker_manager, querier},
    models::index_worker::file_input::FileInputModel,
};
use std::{fs, sync::Arc};
use tantivy::{schema::Schema, Index, IndexReader, IndexWriter};
use tauri::async_runtime::Sender;
use tokio::sync::{mpsc, Mutex};

pub struct SearchIndexService {
    pub schema: Schema,
    pub index_writer: Arc<Mutex<IndexWriter>>,
    index_reader: IndexReader,
    vector_db_service: Arc<VectorDbService>,
}

impl SearchIndexService {
    pub fn new(config: &FileIndexerConfig, vector_db_service: Arc<VectorDbService>) -> Self {
        let app_path = config.app_path.clone();
        let index_path = app_path.join("TantivyOut");

        let schema = file_schema::create_schema();
        // Ensure that the App's AppData directory is there
        if !app_path.exists() {
            fs::create_dir_all("DesktopSearch").expect("could not create DesktopSearch directory");
        }
        // Create the Tantivy index
        let index = if index_path.exists() {
            // If the index directory exists, open the existing index
            println!("Opening existing index at {:?}", index_path);
            Index::open_in_dir(index_path)
        } else {
            // If the index directory doesn't exist, create a new index
            println!("Creating a new index at {:?}", index_path);
            fs::create_dir_all(index_path.clone()).expect("could not create output directory");
            Index::create_in_dir(index_path, schema.clone())
        };
        let index = index.unwrap();
        let index_writer = index.writer(config.buffer_size).unwrap();

        let index_reader = index.reader().unwrap();

        Self {
            schema,
            index_writer: Arc::new(Mutex::new(index_writer)),
            index_reader,
            vector_db_service,
        }
    }

    pub fn query(&self, params: &SearchParamsModel) -> Result<Vec<FileDTO>, tantivy::TantivyError> {
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

        index_worker_manager::spawn_workers(
            receiver,
            index_writer_clone,
            schema_clone,
            db_service,
            vector_processor,
            batch_size,
            8,
        );

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

        index_worker_manager::spawn_workers(
            receiver,
            index_writer_clone,
            schema_clone,
            db_service,
            vector_processor,
            batch_size,
            8,
        );

        sender
    }
}
