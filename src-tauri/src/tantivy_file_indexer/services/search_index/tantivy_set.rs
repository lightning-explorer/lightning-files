use std::sync::Arc;

use tantivy::{schema::Schema, IndexWriter};
use tokio::sync::Mutex;

use crate::tantivy_file_indexer::shared::search_index::tantivy_traits;

/// Helper class to encapsulate a bunch of objects that all share the same Tantivy schema
pub struct TantivySet{
    schema:Schema,
    writer:Arc<Mutex<IndexWriter>>
}