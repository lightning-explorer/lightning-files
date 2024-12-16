use crate::tantivy_file_indexer::dtos::{
    search_params_dto::SearchParamsDTO, streaming_search_dto::StreamingSearchParamsDTO,
};

use super::{
    core::engine::{core::constructor::QueryConstructor, querier::Querier, tantivy_setup},
    models::file::TantivyFileModel,
    services::task_manager::TaskManagerService,
};
use std::{path::PathBuf, sync::Arc};
use tantivy::IndexWriter;

use tauri::{AppHandle, Manager};
use tokio::{sync::Mutex, task::JoinHandle};

pub struct SearchIndexService {
    pub index_writer: Arc<Mutex<IndexWriter>>,
    querier: Arc<Querier>,
}

impl SearchIndexService {
    pub fn new(buffer_size: usize, app_path: PathBuf, handle: &AppHandle) -> Self {
        let index_path = app_path.join("TantivyOut");

        let (schema, index_reader, index_writer) =
            tantivy_setup::initialize_tantity(buffer_size, index_path);

        let index_writer = Arc::new(Mutex::new(index_writer));

        let constructor = Arc::new(QueryConstructor::new(schema.clone(), index_reader.clone()));

        handle.manage(Arc::new(TaskManagerService::new()));

        Self {
            index_writer,
            querier: Arc::new(Querier::new(
                index_reader.clone(),
                Arc::clone(&constructor),
            )),
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
                .advanced_query_streamed(
                    params.params,
                    emit,
                    params.num_events,
                    params.starting_size,
                )
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
                .organized_query_streamed(
                    params.params,
                    emit,
                    params.num_events,
                    params.starting_size,
                )
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
