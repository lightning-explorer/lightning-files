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
};
use std::{fs, sync::Arc};
use tantivy::{schema::Schema, Index, IndexReader, IndexWriter};

use tokio::{sync::Mutex, task::JoinSet};

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
     Returns the sender as well as the handles to the spawned index worker tasks
     */
    pub fn spawn_indexer_db_connected(
        &self,
        db_service: Arc<LocalDbService>,
        batch_size: usize,
        buffer_size: usize,
    ) -> (DbConnectedSender, JoinSet<()>) {
        let schema_clone = Arc::new(self.schema.clone());
        let indexer_table_clone = db_service.indexer_queue_table().clone();
        let (sender, receiver) = db_connected_channel::channel::create(indexer_table_clone);

        let index_writer_clone = self.index_writer.clone();
        let vector_processor = Arc::new(
            self.vector_db_service
                .spawn_indexer(batch_size, buffer_size),
        );

        let indexer_tasks = index_worker_manager::spawn_workers(
            receiver,
            index_writer_clone,
            schema_clone,
            db_service,
            vector_processor,
            batch_size,
            8,
        );

        (sender, indexer_tasks)
    }
}
