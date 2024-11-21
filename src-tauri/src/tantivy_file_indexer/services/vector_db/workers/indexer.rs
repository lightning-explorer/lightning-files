use std::{collections::HashSet, sync::Arc, time::Instant};

use vevtor::{Indexable, Indexer, VevtorService};

use crate::{
    get_directory_path,
    tantivy_file_indexer::{
        dtos::file_dto_input::FileDTOInput,
        services::{
            search_index::models::index_worker::file_input::FileInputModel,
            vector_db::models::embeddable_file_model::EmbeddableFileModel,
        },
    },
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

    pub async fn index_files(&self, model: &FileInputModel, stale_paths: &HashSet<String>) {
        let vector = Arc::clone(&self.vector);
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

        #[cfg(feature="vector_db")]
        println!(
            "removing {} stale entries from vector database",
            stale_paths.len()
        );

        vector
            .delete_by_id(
                stale_paths
                    .iter()
                    .map(|file| (file.collection(), file.get_id()))
                    .collect(),
            )
            .await;
        #[cfg(feature="speed_profile")]    
        let time = Instant::now();

        self.indexer.index(paths).await;

        #[cfg(feature="speed_profile")]    
        println!("Vector index operation took {:?}", time.elapsed());
    }

    fn file_dtos_to_models(&self, dtos: &Vec<&FileDTOInput>) -> Vec<EmbeddableFileModel> {
        dtos.iter()
            .map(|dto| EmbeddableFileModel {
                name: dto.name.clone(),
                parent_dir: get_directory_path(&dto.file_path),
                collection: "files".to_string(),
            })
            .collect()
    }
}
