use crate::tantivy_file_indexer::shared::indexing_crawler::models::file_model::FileModel;

use super::super::entities::file;

impl From<FileModel> for file::Model {
    fn from(val: FileModel) -> Self {
        file::Model {
            path: val.path,
            parent_path: val.parent_path,
        }
    }
}
