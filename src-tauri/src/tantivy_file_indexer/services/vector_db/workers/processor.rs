use std::{collections::HashSet, sync::Arc};

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

pub struct VectorDbProcessor {
    vector: Arc<VevtorService>,
    indexer: Indexer<EmbeddableFileModel>,
}

impl VectorDbProcessor {
    pub fn new(vector: Arc<VevtorService>, batch_size: usize, buffer_size: usize) -> Self {
        let indexer = vector.spawn_index_worker::<EmbeddableFileModel>(batch_size, buffer_size);
        Self { vector, indexer }
    }

    pub async fn process_files(&self, model: &FileInputModel, stale_paths: &HashSet<String>) {
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

        // TODO: remove print
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

        self.embed_files(paths).await;
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

    pub async fn embed_files(&self, files: Vec<EmbeddableFileModel>) {
        self.indexer.index(files).await;
    }
}
