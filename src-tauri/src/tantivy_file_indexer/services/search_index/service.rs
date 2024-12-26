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

use tantivy::{
    indexer::{LogMergePolicy, MergePolicy},
    IndexWriter, SegmentId, SegmentMeta,
};
use tauri::{AppHandle, Manager};
use tokio::{sync::Mutex, task::JoinHandle};

pub struct SearchIndexService {
    /// Dictates how crawlers store documents
    pub pipeline: Arc<TantivyPipeline>,
    querier: Arc<Querier>,
    index: tantivy::Index,
    index_writer: Arc<Mutex<IndexWriter>>,
}

impl SearchIndexService {
    pub fn new(buffer_size: usize, app_path: PathBuf, handle: &AppHandle) -> Self {
        let index_path = app_path.join("TantivyOut");

        let (index, schema, index_reader, index_writer) =
            tantivy_setup::initialize_tantity(buffer_size, index_path);

        let index_writer = Arc::new(Mutex::new(index_writer));

        let constructor = Arc::new(QueryConstructor::new(schema.clone(), index_reader.clone()));

        handle.manage(Arc::new(TaskManagerService::new()));

        // Create the commit pipeline
        let pipeline = TantivyPipeline::new(Arc::clone(&index_writer), index_reader.clone());

        Self {
            pipeline: Arc::new(pipeline),
            querier: Arc::new(Querier::new(index_reader, Arc::clone(&constructor))),
            index,
            index_writer,
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

    /// If the number of `Segments` in the Tantivy Index is greater than `limit`,
    /// then half of all the segments will get merged
    pub async fn merge_segments(&self, limit:usize) -> Result<(), tantivy::TantivyError> {
        let ids = self.get_mergeable_segment_ids(limit)?;
        println!("Search Service: found {} segment ids to merge", ids.len());
        if !ids.is_empty() {
            self.index_writer.lock().await.merge(&ids).await?;
        }
        Ok(())
    }

    fn get_mergeable_segment_ids(&self, limit:usize) -> Result<Vec<SegmentId>, tantivy::TantivyError> {
        let mut ids: Vec<SegmentId> = Vec::new();
        let segments = self.index.searchable_segments()?;
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
