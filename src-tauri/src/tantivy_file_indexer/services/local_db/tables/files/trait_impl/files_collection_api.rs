use crate::tantivy_file_indexer::models::interal_system_file::InternalSystemFileModel;
use crate::tantivy_file_indexer::shared::indexing_crawler::traits::files_collection_api::FilesCollectionApi;

use super::super::api::FilesTable;
use super::super::entities::file;

impl FilesCollectionApi for FilesTable {
    type Error = String;

    fn get_stored_paths(
        &self,
        directory: &std::path::Path,
    ) -> impl std::future::Future<Output = Result<std::collections::HashSet<String>, Self::Error>> + Send
    {
        let dir = directory.to_string_lossy().to_string();
        async move {
            self.get_paths_from_dir(&dir)
                .await
                .map_err(|err| err.to_string())
        }
    }

    fn upsert_many(
        &self,
        models: &[InternalSystemFileModel],
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        let models: Vec<file::Model> = models
            .into_iter()
            .map(|model| model.clone().into())
            .collect();
        async move {
            self.upsert_many(&models)
                .await
                .map_err(|err| err.to_string())
        }
    }

    fn remove_paths(
        &self,
        paths: &std::collections::HashSet<String>,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async move {
            self.remove_paths(paths)
                .await
                .map_err(|err| err.to_string())
        }
    }

    /// All of the other operations automatically commit themselves
    fn commit_all(&self) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async move { Ok(()) }
    }
}
