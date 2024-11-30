use crate::tantivy_file_indexer::{
    services::local_db::tables::crawler_queue::entities::indexed_dir,
    shared::indexing_crawler::{
        models::crawler_file::CrawlerFile, traits::crawler_queue_api::CrawlerQueueApi,
    },
};

use super::super::api::CrawlerQueueTable;

impl CrawlerQueueApi for CrawlerQueueTable {
    type Error = String;

    fn push(
        &self,
        files: &[CrawlerFile],
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        let models: Vec<indexed_dir::Model> = files.iter().map(|x| x.clone().into()).collect();
        async move { self.upsert_many(&models).await.map_err(|err| err.to_string()) }
    }

    fn fetch_next(
        &self,
    ) -> impl std::future::Future<Output = Result<Option<CrawlerFile>, Self::Error>> + Send {
        todo!()
    }

    fn fetch(
        &self,
        amount: u64,
    ) -> impl std::future::Future<Output = Result<Vec<CrawlerFile>, Self::Error>> + Send {
        todo!()
    }

    fn delete_one(
        &self,
        file: CrawlerFile,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        todo!()
    }
}
