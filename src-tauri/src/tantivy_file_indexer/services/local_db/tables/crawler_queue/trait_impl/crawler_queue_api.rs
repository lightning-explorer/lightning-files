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
        async move {
            self.upsert_many(&models)
                .await
                .map_err(|err| err.to_string())
        }
    }

    fn fetch_next(
        &self,
    ) -> impl std::future::Future<Output = Result<Option<CrawlerFile>, Self::Error>> + Send {
        async move {
            let mut models = self.get_many(1).await.map_err(|err| err.to_string())?;
            Ok(models.pop().map(|x| x.into()))
        }
    }

    fn fetch(
        &self,
        amount: u64,
    ) -> impl std::future::Future<Output = Result<Vec<CrawlerFile>, Self::Error>> + Send {
        async move {
            let models = self.get_many(amount).await.map_err(|err| err.to_string())?;
            Ok(models.into_iter().map(|x| x.into()).collect())
        }
    }

    fn delete_one(
        &self,
        file: CrawlerFile,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        let model: indexed_dir::Model = file.into();
        async move{
            self.delete_many(&[model]).await.map_err(|err| err.to_string())
        }
    }

    fn set_taken_to_false_all(
        &self,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async move{
            self.mark_all_as_not_taken().await.map_err(|err| err.to_string())
        }
    }
}
