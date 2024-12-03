use crate::tantivy_file_indexer::{models::{
    search_params_model::SearchParamsModel, tantivy_file_model::TantivyFileModel,
}, services::vector_db::service::VectorDbService};

use super::core::{files_collection::TantivyFilesCollection, querier, tantivy_setup};
use std::{path::PathBuf, sync::Arc};
use tantivy::{schema::Schema, IndexReader, IndexWriter};

use tokio::sync::Mutex;

pub struct SearchIndexService {
    pub schema: Schema,
    pub index_writer: Arc<Mutex<IndexWriter>>,
    index_reader: Arc<IndexReader>,
    vector_db_service: Arc<VectorDbService>,
    pub files_collection:Arc<TantivyFilesCollection>
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

        let index_writer = Arc::new(Mutex::new(index_writer));
        let index_reader = Arc::new(index_reader);

        let files_collection =  Arc::new(TantivyFilesCollection::new(
            Arc::clone(&index_writer), schema.clone(), Arc::clone(&index_reader)));

        Self {
            schema,
            index_writer,
            index_reader,
            vector_db_service,
            files_collection
        }
    }

    pub fn query(
        &self,
        params: &SearchParamsModel,
    ) -> Result<Vec<TantivyFileModel>, tantivy::TantivyError> {
        querier::advanced_query(&self.schema, &self.index_reader.searcher(), params)
    }
}
