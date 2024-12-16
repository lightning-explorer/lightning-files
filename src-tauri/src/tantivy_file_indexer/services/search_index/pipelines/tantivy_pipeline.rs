use std::sync::Arc;

use super::super::core::indexer;
use crate::shared::models::sys_file_model::SystemFileModel;
use crate::tantivy_file_indexer::shared::search_index::tantivy_traits::FromDocument;
use crate::tantivy_file_indexer::{
    services::search_index::models::file::TantivyFileModel,
    shared::indexing_crawler::traits::commit_pipeline::CrawlerCommitPipeline,
};
use tantivy::{query::TermQuery, IndexReader, IndexWriter, TantivyDocument};
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

    /// Helper function to map errors into strings
    fn map_err<T, E: ToString>(result: Result<T, E>) -> Result<T, String> {
        result.map_err(|err| err.to_string())
    }

    /// Helper function to get stale models. The `models` parameter represents the new models and `children` are the old ones
    fn classify_stale_models(
        &self,
        children: &[SystemFileModel],
        models: &[TantivyFileModel],
    ) -> Vec<SystemFileModel> {
        let mut stale = Vec::new();

        for child in children.iter() {
            if !models.iter().any(|file| file.file_path == child.file_path) {
                stale.push(child.clone());
            }
        }
        stale
    }

    fn search_by_parent_directory(
        &self,
        model: &TantivyFileModel,
    ) -> tantivy::Result<Vec<TantivyDocument>> {
        // TODO: do something abou this. Accessing the field directly here is kinda arbitrary
        let term = model.get_field("parent_directory").unwrap();

        // Create a term query
        let query = TermQuery::new(term, tantivy::schema::IndexRecordOption::Basic);

        // Search using the query
        let searcher = self.reader.searcher();
        let top_docs =
            searcher.search(&query, &tantivy::collector::TopDocs::with_limit(1_000_000))?;

        // Collect the document addresses. Use flatten for now since 'searcher.doc' can return an error
        let doc_addresses: Vec<TantivyDocument> = top_docs
            .into_iter()
            .map(|(_, doc_addr)| searcher.doc(doc_addr))
            .flatten()
            .collect();

        Ok(doc_addresses)
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
        let paths = Self::map_err(self.search_by_parent_directory(&parent.clone().into()))?;
        Ok(paths
            .into_iter()
            .map(|doc| TantivyFileModel::from_doc(doc, 0.0).into())
            .collect())
    }

    async fn upsert_many(
        &self,
        models: &[Self::InputModel],
        parent: &Self::InputModel,
    ) -> Result<(), Self::Error> {
        let children = self.get_children(parent).await?;
        let tantivy_models: Vec<TantivyFileModel> =
            models.iter().map(|model| model.clone().into()).collect();

        let stale = self.classify_stale_models(&children, &tantivy_models);

        // Remove stale paths
        self.remove_many(&stale).await?;

        Self::map_err(
            indexer::add_entries_to_index(&tantivy_models, Arc::clone(&self.index_writer)).await,
        )?;

        Ok(())
    }

    async fn upsert_one(&self, model:Self::InputModel) -> Result<(), Self::Error> {
        let model:TantivyFileModel = model.into();
        Self::map_err(
            indexer::add_entries_to_index(&vec![model], Arc::clone(&self.index_writer)).await,
        )?;
        Ok(())
    }

    async fn remove_many(&self, models: &Vec<Self::InputModel>) -> Result<(), Self::Error> {
        if !models.is_empty() {
            println!(
                "Tantivy Pipeline: removing {} models from the index",
                models.len()
            );
            let tantivy_models: Vec<Self::IndexedModel> =
                models.iter().map(|model| model.clone().into()).collect();
            Self::map_err(
                indexer::remove_entries_from_index(tantivy_models, Arc::clone(&self.index_writer))
                    .await,
            )?;
        }
        Ok(())
    }

    async fn commit_all(&self) -> Result<(), Self::Error> {
        Self::map_err(indexer::commit(Arc::clone(&self.index_writer), 3).await)
    }
}
