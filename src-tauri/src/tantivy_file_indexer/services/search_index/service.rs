use crate::{
    shared::models::sys_file_model::SystemFileModel,
    tantivy_file_indexer::{
        dtos::{
            search_params_dto::SearchParamsDTO, streaming_search_dto::StreamingSearchParamsDTO,
        },
        shared::indexing_crawler::traits::commit_pipeline::CrawlerCommitPipeline,
    },
};

use super::{
    core::engine::{querier::Querier, query_builder::constructor::QueryConstructor, tantivy_setup},
    models::file::TantivyFileModel,
    pipelines::tantivy_pipeline::TantivyPipeline,
    services::task_manager::TaskManagerService,
};
use std::{path::PathBuf, sync::Arc};

use tauri::{AppHandle, Manager};
use tokio::{sync::Mutex, task::JoinHandle};

pub struct SearchIndexService {
    /// Dictates how crawlers store documents
    pub pipeline: Arc<TantivyPipeline>,
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

        // Create the commit pipeline
        let pipeline = TantivyPipeline::new(Arc::clone(&index_writer), index_reader.clone());

        Self {
            pipeline: Arc::new(pipeline),
            querier: Arc::new(Querier::new(index_reader, Arc::clone(&constructor))),
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

    pub async fn get_file_from_index(&self, file: SystemFileModel) -> Option<SystemFileModel> {
        self.pipeline.get_one(file).await.map(|model| model.into())
    }

    pub async fn upsert_file_to_index(&self, file: SystemFileModel) -> Result<(), String> {
        self.pipeline.upsert_one(file).await
    }

    pub fn get_pipeline(&self) -> Arc<TantivyPipeline> {
        Arc::clone(&self.pipeline)
    }
}
