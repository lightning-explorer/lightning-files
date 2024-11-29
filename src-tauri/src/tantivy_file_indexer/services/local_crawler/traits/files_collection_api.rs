// TODO: put with the rest of the database code

use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    future::Future,
    path::Path,
};

use crate::tantivy_file_indexer::services::local_crawler::models::file_model::FileModel;

pub trait FilesCollectionApi: Clone + Send + Sync + 'static {
    type Error: Display + Debug;

    /**
    Get all of the paths that exist inside the specified directory
     */
    fn get_stored_paths(
        &self,
        directory: &Path,
    ) -> impl Future<Output = Result<HashSet<String>, Self::Error>> + Send;

    fn upsert_many<T>(&self, models: &[T]) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        T: Into<FileModel>;

    fn remove_paths<'a, I, S>(
        &self,
        paths: I,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str> + 'a;
}
