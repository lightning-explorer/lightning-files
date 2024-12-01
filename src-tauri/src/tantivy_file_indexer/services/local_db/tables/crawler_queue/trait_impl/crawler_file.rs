use std::path::PathBuf;

use crate::tantivy_file_indexer::shared::indexing_crawler::models::crawler_file::CrawlerFile;

use super::super::entities::indexed_dir;

impl From<CrawlerFile> for indexed_dir::Model {
    fn from(val:CrawlerFile) -> indexed_dir::Model {
        indexed_dir::Model {
            path: val.path.to_string_lossy().to_string(),
            priority: val.priority,
            taken: val.taken,
        }
    }
}

impl From<indexed_dir::Model> for CrawlerFile{
    fn from(val:indexed_dir::Model) -> CrawlerFile {
        CrawlerFile {
            path: PathBuf::from(val.path),
            priority: val.priority,
            taken: val.taken,
        }
    }
}
