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
    core::engine::{querier::Querier, query_builder::constructor::QueryConstructor},
    models::file::TantivyFileModel,
    pipelines::tantivy_pipeline::TantivyPipeline,
    services::task_manager::TaskManagerService,
};
use std::{path::PathBuf, sync::Arc};

use tantivy::directory::GarbageCollectionResult;
use tantivy_ext::SearchIndex;
use tauri::{AppHandle, Manager};
use tokio::task::JoinHandle;

pub struct SearchIndexService {
    /// Dictates how crawlers store documents
    pub pipeline: Arc<TantivyPipeline>,
    querier: Arc<Querier>,
    index: SearchIndex<TantivyFileModel>,
}

impl SearchIndexService {
    pub fn new(app_path: PathBuf, handle: &AppHandle) -> Self {
        let index_path = app_path.join("TantivyOut");

        let index = SearchIndex::new(50_000_000, index_path);
        let backend = index.get_tantivy_backend();

        let constructor = Arc::new(QueryConstructor::new(
            backend.schema,
            backend.reader.clone(),
        ));

        handle.manage(Arc::new(TaskManagerService::new()));

        // Create the commit pipeline
        let pipeline = TantivyPipeline::new(index.clone());

        Self {
            pipeline: Arc::new(pipeline),
            querier: Arc::new(Querier::new(
                backend.reader.clone(),
                Arc::clone(&constructor),
            )),
            index,
        }
    }

    /// Spawns a tokio task for the query
    pub fn streaming_query<EmitFn>(
        &self,
        params: StreamingSearchParamsDTO,
        emit: EmitFn,
    ) -> JoinHandle<tantivy::Result<()>>
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

    /// Removes entries not in use anymore
    pub async fn collect_garbage(&self) -> Result<GarbageCollectionResult, tantivy::TantivyError> {
        let backend = self.index.get_tantivy_backend();
        let mut writer_lock = backend.writer.lock().await;
        if let Err(err) = writer_lock.commit(){
            println!("SearchService: Garbage collection failed to commit pending changes: {}",err);
        }
        writer_lock.garbage_collect_files().await
    }
}
