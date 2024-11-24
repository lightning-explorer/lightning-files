use crate::tantivy_file_indexer::services::{
    local_db::tables::indexer_queue::entities::directory_payload,
    search_index::models::index_worker::file_input::FileInputModel,
};

impl From<FileInputModel> for directory_payload::Model {
    fn from(val: FileInputModel) -> Self {
        directory_payload::Model {
            id: 0,
            directory_from: val.directory_from.to_string_lossy().to_string(),
            files: serde_json::to_value(val.dtos).unwrap(),
        }
    }
}
