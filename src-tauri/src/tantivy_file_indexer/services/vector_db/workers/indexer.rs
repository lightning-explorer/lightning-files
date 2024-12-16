use std::{collections::HashSet, sync::Arc, time::Instant};

use vevtor::{Indexable, Indexer, VevtorService};

use crate::{
    get_directory_path, shared::models::sys_file_model::SystemFileModel, tantivy_file_indexer::{
        services::vector_db::models::embeddable_file_model::EmbeddableFileModel,
        shared::indexing_crawler::models::system_directory_model::InternalSystemDirectoryModel,
    }
};

pub struct VectorDbIndexer {
    vector: Arc<VevtorService>,
    indexer: Indexer<EmbeddableFileModel>,
}

impl VectorDbIndexer {
    pub fn new(vector: Arc<VevtorService>, batch_size: usize, buffer_size: usize) -> Self {
        let indexer = vector.spawn_index_worker::<EmbeddableFileModel>(batch_size, buffer_size);
        Self { vector, indexer }
    }

    /**
    Returns a handle to the indexing task that got spawned
    */
    pub async fn index_files(
        &self,
        model: &InternalSystemDirectoryModel,
        stale_paths: &HashSet<String>,
    ) -> tokio::task::JoinHandle<()> {
        let vector_clone = Arc::clone(&self.vector);
        let indexer_clone = self.indexer.clone();
        let paths = self.file_dtos_to_models(
            &model
                .dtos
                .iter()
                .filter(|x| !stale_paths.contains(&x.file_path))
                .collect(),
        );
        let stale_paths = self.file_dtos_to_models(
            &model
                .dtos
                .iter()
                .filter(|x| stale_paths.contains(&x.file_path))
                .collect(),
        );

        tokio::task::spawn(async move {
            #[cfg(feature = "vector_db_logs")]
            println!(
                "removing {} stale entries from vector database",
                stale_paths.len()
            );

            vector_clone
                .delete_by_id(
                    stale_paths
                        .iter()
                        .map(|file| (file.collection(), file.get_id()))
                        .collect(),
                )
                .await;

            indexer_clone.index(paths).await;
        })
    }

    fn file_dtos_to_models(
        &self,
        dtos: &Vec<&SystemFileModel>,
    ) -> Vec<EmbeddableFileModel> {
        dtos.iter()
            .map(|dto| EmbeddableFileModel {
                name: dto.name.clone(),
                parent_dir: get_directory_path(&dto.file_path),
                collection: "files".to_string(),
            })
            .collect()
    }
}
