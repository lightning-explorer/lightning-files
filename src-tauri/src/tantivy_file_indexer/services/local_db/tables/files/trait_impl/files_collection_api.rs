use crate::tantivy_file_indexer::shared::indexing_crawler::{
    models::file_model::FileModel, traits::files_collection_api::FilesCollectionApi,
};

use super::super::entities::file;
use super::super::api::FilesTable;

impl FilesCollectionApi for FilesTable {
    type Error = String;

    fn get_stored_paths(
        &self,
        directory: &std::path::Path,
    ) -> impl std::future::Future<Output = Result<std::collections::HashSet<String>, Self::Error>> + Send
    {
        let dir = directory.to_string_lossy().to_string();
        async move { self.get_paths_from_dir(&dir).await.map_err(|err|err.to_string()) }
    }

    fn upsert_many(
        &self,
        models: Vec<FileModel>,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send
    {
        let models: Vec<file::Model> = models.into_iter().map(|model| model.into()).collect();
        async move { self.upsert_many(&models).await.map_err(|err|err.to_string()) }
    }

    fn remove_paths(
        &self,
        paths: &std::collections::HashSet<String>,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async move{
            self.remove_paths(paths).await.map_err(|err|err.to_string())
        }
    }
}
