use std::sync::Arc;

use vevtor::{VectorQueryModel, VevtorService};

use super::{
    models::embeddable_file_model::{self},
    workers::indexer::{self, VectorDbIndexer},
};

pub struct VectorDbService {
    vevtor: Arc<VevtorService>,
}

impl VectorDbService {
    pub fn new() -> Self {
        let url = "http://127.0.0.1:6334";
        let vevtor = Arc::new(VevtorService::new(url));
        Self { vevtor }
    }

    pub async fn delete_all_collections(&self) {
        self.vevtor.delete_all_collections().await;
    }

    pub async fn list_collections(&self) -> Vec<String> {
        self.vevtor.list_collections().await
    }

    pub async fn query(
        &self,
        query: &str,
        collection: &str,
    ) -> Result<Vec<(embeddable_file_model::EmbeddableFileModel, f32)>, String> {
        self.vevtor
            .search(
                &VectorQueryModel {
                    collection: collection.to_string(),
                    query: query.to_string(),
                },
                10,
            )
            .await
    }

    pub fn spawn_indexer(&self, batch_size: usize, buffer_size: usize) -> VectorDbIndexer {
        let vevtor_clone = Arc::clone(&self.vevtor);
        indexer::VectorDbIndexer::new(vevtor_clone, batch_size, buffer_size)
    }
}
