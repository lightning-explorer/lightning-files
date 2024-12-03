use crate::tantivy_file_indexer::{services::local_db::tables::indexer_queue::entities::directory_payload, shared::indexing_crawler::models::system_directory_model::InternalSystemDirectoryModel};

impl From<InternalSystemDirectoryModel> for directory_payload::Model {
    fn from(val: InternalSystemDirectoryModel) -> Self {
        directory_payload::Model {
            id: 0,
            directory_from: val.path.to_string_lossy().to_string(),
            files: serde_json::to_value(val.dtos).unwrap(),
        }
    }
}
