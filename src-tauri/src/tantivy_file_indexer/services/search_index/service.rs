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

use tantivy::SegmentId;
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

    /// If the number of `Segments` in the Tantivy Index is greater than `limit`,
    /// then half of all the segments will get merged
    pub async fn merge_segments(&self, limit: usize) -> Result<(), tantivy::TantivyError> {
        let ids = self.get_mergeable_segment_ids(limit)?;
        println!("Search Service: found {} segment ids to merge", ids.len());
        if !ids.is_empty() {
            self.index
                .get_tantivy_backend()
                .writer
                .lock()
                .await
                .merge(&ids)
                .await?;
        }
        Ok(())
    }

    fn get_mergeable_segment_ids(
        &self,
        limit: usize,
    ) -> Result<Vec<SegmentId>, tantivy::TantivyError> {
        let mut ids: Vec<SegmentId> = Vec::new();
        let segments = self
            .index
            .get_tantivy_backend()
            .index
            .searchable_segments()?;
        if segments.len() > limit {
            for (i, segment) in segments.iter().enumerate() {
                if i % 2 == 0 {
                    ids.push(segment.id());
                }
            }
        }
        Ok(ids)
    }
}
