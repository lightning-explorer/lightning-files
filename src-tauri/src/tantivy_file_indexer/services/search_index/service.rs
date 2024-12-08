use crate::tantivy_file_indexer::{
    dtos::{search_params_dto::SearchParamsDTO, streaming_search_dto::StreamingSearchParamsDTO},
    models::tantivy_file_model::TantivyFileModel,
};

use super::{
    core::{query::querier, tantivy_setup},
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

        Self {
            schema,
            index_writer,
            index_reader,
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
        let schema = self.schema.clone();
        let searcher = self.index_reader.searcher();
        let search_params = params.params;
        let step_size = params.num_events;
        let min_results = params.starting_size;

        tokio::spawn(async move {
            querier::advanced_query_streamed(
                schema,
                searcher,
                search_params,
                emit,
                step_size,
                min_results,
            )
            .await
        })
    }

    pub fn query(
        &self,
        params: &SearchParamsDTO,
    ) -> Result<Vec<TantivyFileModel>, tantivy::TantivyError> {
        querier::advanced_query(&self.schema, &self.index_reader.searcher(), params)
    }
}
