use crate::get_directory_path;
use crate::tantivy_file_indexer::dtos::file_dto_input::FileDTOInput;
use crate::tantivy_file_indexer::services::search_index::models::index_worker::file_input::FileInputModel;

use super::core::indexer_api::models::search_query_models::VectorQueryModel;
use super::core::indexer_api::service::VevtorService;
use super::models::file_model;

pub struct VectorDbService {
    vevtor: VevtorService<file_model::FileModel>,
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

    pub async fn embed_files(&self, dtos: &Vec<FileDTOInput>) {
        let files = Self::file_dtos_to_models(dtos);
        self.vevtor.add_files(files).await;
    }

    pub async fn query(
        &self,
        query: &str,
        collection: &str,
    ) -> Result<Vec<(file_model::FileModel, f32)>, String> {
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

    fn file_dtos_to_models(dtos: &Vec<FileDTOInput>) -> Vec<file_model::FileModel> {
        dtos
            .iter()
            .map(|dto| file_model::FileModel {
                name: dto.name.clone(),
                parent_dir: get_directory_path(&dto.file_path),
                collection: "files".to_string(),
            })
            .collect()
    }
}
