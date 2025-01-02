use std::sync::Arc;

use super::{super::core::indexer, util};
use tantivy::IndexWriter;
use tokio::sync::Mutex;

use crate::{
    shared::models::sys_file_model::SystemFileModel,
    tantivy_file_indexer::{
        services::{
            local_db::tables::files::{api::FilesTable, entities::file},
            search_index::models::file::TantivyFileModel,
        },
        shared::indexing_crawler::traits::commit_pipeline::CrawlerCommitPipeline,
    },
};

/// Commits files to the Tantivy index and also stores other files in the database
pub struct DbTantivyPipeline {
    files_table: FilesTable,
    index_writer: Arc<Mutex<IndexWriter>>,
}

impl DbTantivyPipeline {
    pub fn new(files_table: FilesTable, index_writer: Arc<Mutex<IndexWriter>>) -> Self {
        Self {
            files_table,
            index_writer,
        }
    }

    /// Helper function to map errors into strings
    fn map_err<T, E: ToString>(result: Result<T, E>) -> Result<T, String> {
        result.map_err(|err| err.to_string())
    }

    /// Helper function to get stale models. The `models` parameter represents the new models and `children` are the old ones
    fn classify_stale_models(
        &self,
        children_keys: Vec<String>,
        models: &[SystemFileModel],
    ) -> Vec<String> {
        let mut stale = Vec::new();

        for key in children_keys.iter() {
            if !models.iter().any(|file| file.file_path == key) {
                stale.push(key.clone());
            }
        }
        stale
    }
}

impl CrawlerCommitPipeline for DbTantivyPipeline {
    type IndexedModel = TantivyFileModel;
    type InputModel = SystemFileModel;
    type Error = String;

    async fn get_children_keys(
        &self,
        parent: &Self::InputModel,
    ) -> Result<Vec<String>, Self::Error> {
        let dir = parent.file_path.clone();
        let paths = Self::map_err(self.files_table.get_paths_from_dir(&dir).await)?;
        Ok(paths.into_iter().collect())
    }

    async fn upsert_many(
        &self,
        models: &[Self::InputModel],
        parent: &Self::InputModel,
    ) -> Result<(), Self::Error> {
        let children = self.get_children_keys(parent).await?;
        let stale = self.classify_stale_models(&children, models);

        // Remove stale paths
        self.remove_many(&stale).await?;

        let tantivy_models: Vec<TantivyFileModel> =
            models.iter().map(|model| model.clone().into()).collect();
        Self::map_err(
            indexer::add_entries_to_index(&tantivy_models, Arc::clone(&self.index_writer)).await,
        )?;

        // Upsert new entries into the database and index
        let db_models: Vec<file::Model> = models.iter().map(|model| model.clone().into()).collect();
        Self::map_err(self.files_table.upsert_many(&db_models).await)?;

        Ok(())
    }

    async fn upsert_one(&self, model:Self::InputModel) -> Result<(), Self::Error> {
        let model:TantivyFileModel = model.into();
        Self::map_err(
            indexer::add_entries_to_index(&[model], Arc::clone(&self.index_writer)).await,
        )?;
        Ok(())
    }

    async fn get_one(&self, model: Self::InputModel) -> Option<Self::IndexedModel> {
        // TODO: implement
        None
    }

    async fn remove_many(&self, models: &Vec<Self::InputModel>) -> Result<(), Self::Error> {
        let paths: Vec<String> = models.iter().map(|model| model.file_path.clone()).collect();
        Self::map_err(self.files_table.remove_paths(paths).await)?;

        let tantivy_models: Vec<Self::IndexedModel> =
            models.iter().map(|model| model.clone().into()).collect();
        Self::map_err(
            indexer::remove_entries_from_index(tantivy_models, Arc::clone(&self.index_writer))
                .await,
        )
    }
}
