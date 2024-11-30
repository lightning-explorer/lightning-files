use crate::tantivy_file_indexer::shared::indexing_crawler::models::crawler_file::CrawlerFile;

use super::super::entities::indexed_dir;

impl Into<indexed_dir::Model> for CrawlerFile {
    fn into(self) -> indexed_dir::Model {
        indexed_dir::Model {
            path: self.path.to_string_lossy().to_string(),
            priority: self.priority,
        }
    }
}
