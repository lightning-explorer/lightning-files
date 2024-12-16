use std::sync::Arc;

use super::super::core::indexer;
use super::util;
use crate::shared::models::sys_file_model::SystemFileModel;
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
        let paths = util::map_err(util::search_by_parent_directory(
            self.reader.searcher(),
            &parent.clone().into(),
        ))?;
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

        let stale = util::classify_stale_models(&children, &tantivy_models);

        // Remove stale paths
        self.remove_many(&stale).await?;

        util::map_err(
            indexer::add_entries_to_index(&tantivy_models, Arc::clone(&self.index_writer)).await,
        )?;

        Ok(())
    }

    async fn upsert_one(&self, model: Self::InputModel) -> Result<(), Self::Error> {
        let model: TantivyFileModel = model.into();
        util::map_err(
            indexer::add_entries_to_index(&vec![model], Arc::clone(&self.index_writer)).await,
        )?;
        Ok(())
    }

    async fn get_one(&self, model: Self::InputModel) -> Option<Self::IndexedModel> {
        match util::search_by_path(self.reader.searcher(), &model.file_path).unwrap_or(None) {
            Some(doc) => Some(TantivyFileModel::from_doc(doc, 0.0)),
            None => None,
        }
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

    async fn commit_all(&self) -> Result<(), Self::Error> {
        util::map_err(indexer::commit(Arc::clone(&self.index_writer), 3).await)
    }
}
