use std::collections::HashMap;

use qdrant_client::qdrant::Value;
use serde::{Deserialize, Serialize};

use crate::tantivy_file_indexer::services::vevtor::core::indexer_api::traits::indexable::Indexable;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileModel {
    pub name: String,
    pub collection: String,
}

impl Indexable for FileModel {
    type Output = FileModel;

    fn as_map(&self) -> HashMap<String, Value> {
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert("name".to_string(), self.name.clone().into());
        map
    }

    fn from_qdrant_payload(
        payload: &std::collections::HashMap<String, qdrant_client::qdrant::Value>,
        collection: String,
    ) -> Result<FileModel, String> {
        payload
            .get("name")
            .and_then(|name_key| name_key.as_str().map(|name| name.to_string()))
            .map(|name| FileModel { name, collection })
            .ok_or_else(|| "Name field doesn't exist".to_string())
    }

    fn get_id(&self) -> u64 {
        string_to_u64(&self.name)
    }

    fn embed_label(&self)->&str{
        &self.name
    }

    fn collection(&self) -> String {
        self.collection.to_string()
    }
}

use twox_hash::XxHash64;
use std::hash::{Hash, Hasher};

pub fn string_to_u64(s: &str) -> u64 {
    let mut hasher = XxHash64::default();
    s.hash(&mut hasher);
    hasher.finish()
}
