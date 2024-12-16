use crate::{shared::models::sys_file_model::SystemFileModel, tantivy_file_indexer::{dtos::{
    search_params_dto::SearchParamsDTO, streaming_search_dto::StreamingSearchParamsDTO,
}, services::local_db::service::LocalDbService, shared::indexing_crawler::traits::commit_pipeline::CrawlerCommitPipeline}};

use super::{
    core::engine::{core::constructor::QueryConstructor, querier::Querier, tantivy_setup}, models::file::TantivyFileModel, pipelines::{db_tantivy_pipeline::DbTantivyPipeline, tantivy_pipeline::TantivyPipeline}, services::task_manager::TaskManagerService
};
use std::{path::PathBuf, sync::Arc};
use tantivy::{IndexReader, IndexWriter};

use tauri::{AppHandle, Manager};
use tokio::{sync::Mutex, task::JoinHandle};

pub enum PipelineType{
    Db,
    Tantivy
}
type Pipeline = Box<dyn CrawlerCommitPipeline<IndexedModel = TantivyFileModel, InputModel = SystemFileModel, Error = String>>;
pub struct SearchIndexService {
    pub index_writer: Arc<Mutex<IndexWriter>>,
    pub index_reader: IndexReader,
    /// Dictates how crawlers store documents
    pub pipeline: Arc<Pipeline>,
    querier: Arc<Querier>,
}

impl SearchIndexService {
    pub fn new(buffer_size: usize, app_path: PathBuf, handle: &AppHandle, db_service:Arc<LocalDbService>, pipeline_type:PipelineType) -> Self {
        let index_path = app_path.join("TantivyOut");

        let (schema, index_reader, index_writer) =
            tantivy_setup::initialize_tantity(buffer_size, index_path);

        let index_writer = Arc::new(Mutex::new(index_writer));

        let constructor = Arc::new(QueryConstructor::new(schema.clone(), index_reader.clone()));

        handle.manage(Arc::new(TaskManagerService::new()));

        // Create the commit pipeline
        let pipeline: Pipeline = match pipeline_type{
            PipelineType::Db =>{
                Box::new(DbTantivyPipeline::new(db_service.files_table().clone(),Arc::clone(&index_writer)))
            }
            PipelineType::Tantivy =>{
                Box::new(TantivyPipeline::new(Arc::clone(&index_writer), index_reader.clone()))
            }
        };

        Self {
            index_writer,
            index_reader: index_reader.clone(),
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

    pub fn get_pipeline(&self)->Arc<Pipeline>{
        Arc::clone(&self.pipeline)
    }
}
