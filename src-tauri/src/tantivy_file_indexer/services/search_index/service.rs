use crate::tantivy_file_indexer::{
    dtos::{search_params_dto::SearchParamsDTO, streaming_search_dto::StreamingSearchParamsDTO},
    models::tantivy_file_model::TantivyFileModel,
};

use super::{
    core::{query::querier::Querier, tantivy_setup},
    files_collection::TantivyFilesCollection,
    services::task_manager::TaskManagerService,
};
use std::{path::PathBuf, sync::Arc};
use tantivy::{schema::Schema, IndexReader, IndexWriter};

use tauri::{AppHandle, Manager};
use tokio::{sync::Mutex, task::JoinHandle};

pub struct SearchIndexService {
    pub schema: Schema,
    pub index_writer: Arc<Mutex<IndexWriter>>,
    index_reader: Arc<IndexReader>,
    pub files_collection: Arc<TantivyFilesCollection>,
    querier: Arc<Querier>,
}

impl SearchIndexService {
    pub fn new(buffer_size: usize, app_path: PathBuf, handle: &AppHandle) -> Self {
        let index_path = app_path.join("TantivyOut");

        let (schema, index_reader, index_writer) =
            tantivy_setup::initialize_tantity(buffer_size, index_path);

        let index_writer = Arc::new(Mutex::new(index_writer));
        let index_reader = Arc::new(index_reader);

        let files_collection = Arc::new(TantivyFilesCollection::new(
            Arc::clone(&index_writer),
            schema.clone(),
            Arc::clone(&index_reader),
        ));

        handle.manage(Arc::new(TaskManagerService::new()));
        let schema_clone = schema.clone();
        let searcher = index_reader.searcher();

        Self {
            schema,
            index_writer,
            index_reader,
            querier: Arc::new(Querier::new(schema_clone,searcher)),
            files_collection,
        }
    }

    /// Spawns a tokio task for the query
    pub fn streaming_query<EmitFn>(
        &self,
        params: StreamingSearchParamsDTO,
        emit: EmitFn,
    ) -> JoinHandle<()>
    where
        EmitFn: Fn(Vec<TantivyFileModel>) + Send + 'static,
    {
        let querier_clone = Arc::clone(&self.querier);
        tokio::spawn(async move {
            querier_clone
                .advanced_query_streamed(params.params, emit, params.num_events, params.starting_size)
                .await
        })
    }

    /// Spawns a tokio task for the query
    pub fn streaming_query_organized<EmitFn>(
        &self,
        params: StreamingSearchParamsDTO,
        emit: EmitFn,
    ) -> JoinHandle<()>
    where
        EmitFn: Fn(&[TantivyFileModel]) + Send + 'static,
    {
        let querier_clone = Arc::clone(&self.querier);
        tokio::spawn(async move {
            querier_clone
                .organized_query_streamed(params.params, emit, params.num_events, params.starting_size)
                .await
        })
    }

    pub fn query(
        &self,
        params: &SearchParamsDTO,
    ) -> Result<Vec<TantivyFileModel>, tantivy::TantivyError> {
        self.querier.advanced_query(params)
    }
}
