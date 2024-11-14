use vevtor::{VectorQueryModel, VevtorService};

use crate::get_directory_path;
use crate::tantivy_file_indexer::dtos::file_dto_input::FileDTOInput;

use super::models::embeddable_file_model::{self, EmbeddableFileModel};

pub struct VectorDbService {
    vevtor: VevtorService<embeddable_file_model::EmbeddableFileModel>,
}

impl VectorDbService {
    pub fn new() -> Self {
        let url = "http://127.0.0.1:6334";
        let vevtor = VevtorService::new(url, 64);
        Self { vevtor }
    }

    pub async fn delete_all_collections(&self) {
        self.vevtor.delete_all_collections().await;
    }

    pub async fn list_collections(&self) -> Vec<String> {
        self.vevtor.list_collections().await
    }

    pub async fn delete_by_id(&self, items: Vec<u64>) {
        self.vevtor
            .delete_by_id(
                items
                    .into_iter()
                    .map(|x| ("files".to_string(), x))
                    .collect(),
            )
            .await;
    }

    pub fn files_to_models(&self, dtos: &Vec<&FileDTOInput>) -> Vec<EmbeddableFileModel> {
        Self::file_dtos_to_models(dtos)
    }

    pub async fn embed_files(&self, files: Vec<EmbeddableFileModel>) {
        self.vevtor.add_files(files).await;
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

    fn file_dtos_to_models(dtos: &Vec<&FileDTOInput>) -> Vec<embeddable_file_model::EmbeddableFileModel> {
        dtos.iter()
            .map(|dto| embeddable_file_model::EmbeddableFileModel {
                name: dto.name.clone(),
                parent_dir: get_directory_path(&dto.file_path),
                collection: "files".to_string(),
            })
            .collect()
    }
}
