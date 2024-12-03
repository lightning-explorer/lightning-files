use crate::tantivy_file_indexer::{models::{
    search_params_model::SearchParamsModel, tantivy_file_model::TantivyFileModel,
}, services::vector_db::service::VectorDbService};

use super::core::{querier, tantivy_setup};
use std::{path::PathBuf, sync::Arc};
use tantivy::{schema::Schema, IndexReader, IndexWriter};

use tokio::sync::Mutex;

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
}
