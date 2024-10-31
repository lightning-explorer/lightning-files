use crate::{
    shared::dtos::file_dto::FileDTO,
    tantivy_file_indexer::models::search_params_model::SearchParamsModel,
};

use super::{
    super::{configs::file_indexer_config::FileIndexerConfig, schemas::file_schema},
    querier,
};
use dirs::data_dir;
use std::{fs, sync::Arc};
use tantivy::{schema::Schema, Index, IndexReader, IndexWriter};
use tokio::sync::Mutex;

pub struct SearchIndexService {
    pub schema: Schema,
    pub index_writer: Arc<Mutex<IndexWriter>>,
    index_reader: IndexReader,
}

impl SearchIndexService {
    pub fn new(config: &FileIndexerConfig) -> Self {
        let app_data = data_dir().expect("Could not find AppData directory");
        let app_path = app_data.join("DesktopSearch");
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
        }
    }

    pub fn query(&self, params: &SearchParamsModel) -> Result<Vec<FileDTO>, tantivy::TantivyError> {
        querier::advanced_query(&self.schema, &self.index_reader.searcher(), params)
    }
}
