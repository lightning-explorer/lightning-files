// TODO: put with the rest of the database code

use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    future::Future,
    path::Path,
};

use super::super::models::file_model::FileModel;
pub trait FilesCollectionApi: Clone + Send + Sync + 'static {
    type Error: Display + Debug;

    /**
    Get all of the paths that exist inside the specified directory
     */
    fn get_stored_paths(
        &self,
        directory: &Path,
    ) -> impl Future<Output = Result<HashSet<String>, Self::Error>> + Send;

    fn upsert_many(&self, models: Vec<FileModel>) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn remove_paths(
        &self,
        paths: &HashSet<String>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}