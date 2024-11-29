use std::{
    fmt::{Debug, Display},
    future::Future,
};

use super::super::models::crawler_file::CrawlerFile;

pub trait CrawlerQueueApi: Clone + Send + Sync + 'static {
    type Error: Display + Debug;

    fn push<T>(&self, files: &[T]) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        T: Into<CrawlerFile> + Send;

    /**
    Get the next files from the database without removing them
    */
    fn fetch(
        &self,
        amount: u64,
    ) -> impl Future<Output = Result<Vec<CrawlerFile>, Self::Error>> + Send;

    fn delete<T>(&self, files: &[T]) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        T: Into<CrawlerFile> + Send;
}
