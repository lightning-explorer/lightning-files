use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use crate::tantivy_file_indexer::{db::sqlx_service::SqlxService, service::search_index_service::SearchIndexService};

pub struct FileCrawlerService{
    search_service: Arc<SearchIndexService>,
    db_service: Arc<SqlxService>,
}