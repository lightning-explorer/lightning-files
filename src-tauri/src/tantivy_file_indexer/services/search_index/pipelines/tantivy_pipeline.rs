use std::collections::HashMap;
use std::sync::Arc;

use super::super::core::indexer;
use super::util;
use crate::shared::models::sys_file_model::SystemFileModel;
use crate::tantivy_file_indexer::services::search_index::core::engine::ranker;
use crate::tantivy_file_indexer::shared::search_index::tantivy_traits::FromDocument;
use crate::tantivy_file_indexer::{
    services::search_index::models::file::TantivyFileModel,
    shared::indexing_crawler::traits::commit_pipeline::CrawlerCommitPipeline,
};

use tantivy::{IndexReader, IndexWriter};
use tokio::sync::Mutex;

/// Pipeline where Tantivy is used as the main database. SQLite is used as a queue
pub struct TantivyPipeline {
    index_writer: Arc<Mutex<IndexWriter>>,
    reader: IndexReader,
}

impl TantivyPipeline {
    pub fn new(index_writer: Arc<Mutex<IndexWriter>>, reader: IndexReader) -> Self {
        Self {
            index_writer,
            reader,
        }
    }
}

impl CrawlerCommitPipeline for TantivyPipeline {
    type IndexedModel = TantivyFileModel;
    type InputModel = SystemFileModel;
    type Error = String;

    async fn get_children(
        &self,
        parent: &Self::InputModel,
    ) -> Result<Vec<Self::InputModel>, Self::Error> {
        let paths = util::map_err(util::search_by_directory(
            self.reader.searcher(),
            &parent.clone().into(),
        ))?;

        Ok(paths
            .into_iter()
            .map(|doc| TantivyFileModel::from_doc(doc, 0.0).into())
            .collect())
    }

    /// Upsert files and rank them as well
    async fn upsert_many(
        &self,
        models: &[Self::InputModel],
        parent: &Self::InputModel,
    ) -> Result<(), Self::Error> {
        let children = self.get_children(parent).await?;

        // Create a HashMap for quick lookup of children by `file_path`
        let child_map: HashMap<&str, &SystemFileModel> = children
            .iter()
            .map(|child| (child.file_path.as_str(), child))
            .collect();

        // Separate models into existing and brand_new
        let mut existing: Vec<(&SystemFileModel, &SystemFileModel)> = Vec::new();
        let mut brand_new: Vec<&SystemFileModel> = Vec::new();

        for model in models {
            if let Some(child) = child_map.get(model.file_path.as_str()) {
                existing.push((model, *child));
            } else {
                brand_new.push(model);
            }
        }

        // Rank the files as a part of preprocessing
        let tantivy_models: Vec<TantivyFileModel> = rank_files(existing, brand_new);

        // Classify and remove stale files
        let stale = util::classify_stale_models(&children, &tantivy_models);
        self.remove_many(&stale).await?;

        // Add new entries to the index
        util::map_err(
            indexer::add_entries_to_index(&tantivy_models, Arc::clone(&self.index_writer)).await,
        )?;

        Ok(())
    }

    async fn upsert_one(&self, model: Self::InputModel) -> Result<(), Self::Error> {
        let model: TantivyFileModel = model.into();
        util::map_err(
            indexer::add_entries_to_index(&[model], Arc::clone(&self.index_writer)).await,
        )?;
        Ok(())
    }

    async fn get_one(&self, model: Self::InputModel) -> Option<Self::IndexedModel> {
        util::search_by_path(self.reader.searcher(), &model.file_path)
            .unwrap_or(None)
            .map(|doc| TantivyFileModel::from_doc(doc, 0.0))
    }

    async fn remove_many(&self, models: &Vec<Self::InputModel>) -> Result<(), Self::Error> {
        if !models.is_empty() {
            println!(
                "Tantivy Pipeline: removing {} models from the index",
                models.len()
            );
            let tantivy_models: Vec<Self::IndexedModel> =
                models.iter().map(|model| model.clone().into()).collect();
            util::map_err(
                indexer::remove_entries_from_index(tantivy_models, Arc::clone(&self.index_writer))
                    .await,
            )?;
        }
        Ok(())
    }
}

/// Returns the files you passed in, aggregated and ranked
fn rank_files(existing: Vec<(&SystemFileModel, &SystemFileModel)>, brand_new: Vec<&SystemFileModel>) -> Vec<TantivyFileModel>{
    let mut tantivy_models: Vec<TantivyFileModel> = Vec::new();
    tantivy_models.extend(
        existing
            .into_iter()
            .map(|(new_file, old_file)| ranker::rank_existing_file(new_file, old_file).into()),
    );

    tantivy_models.extend(
        brand_new
            .into_iter()
            .map(|file| ranker::rank_new_file(file).into()),
    );
    tantivy_models
}