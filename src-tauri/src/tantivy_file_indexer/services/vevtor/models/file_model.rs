use super::super::core::indexer_api::traits::indexable::Indexable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FileModel {
    pub name: String,
    pub parent_dir: String,
    pub collection: String,
}

impl Indexable for FileModel {
    type Output = FileModel;

    fn as_map(&self) -> HashMap<String, qdrant_client::qdrant::Value> {
        let value: serde_json::Value = serde_json::to_value(&self).expect("Serialization failed");

        if let serde_json::Value::Object(map) = value {
            map.into_iter().map(|(k, v)| (k, v.into())).collect()
        } else {
            HashMap::new()
        }
    }

    fn from_qdrant_payload(
        payload: &std::collections::HashMap<String, qdrant_client::qdrant::Value>,
        collection: String,
    ) -> Result<FileModel, String> {
        let name = get_value(payload, "name");
        let parent_dir = get_value(payload, "parent_dir");
        Ok(FileModel {
            name: name.to_string(),
            parent_dir: parent_dir.to_string(),
            collection,
        })
    }

    fn get_id(&self) -> u64 {
        string_to_u64(&self.name)
    }

    fn embed_label(&self) -> &str {
        &self.name
    }

    fn collection(&self) -> String {
        self.collection.to_string()
    }
}

fn get_value(
    payload: &std::collections::HashMap<String, qdrant_client::qdrant::Value>,
    field: &str,
) -> qdrant_client::qdrant::Value {
    payload.get(field).expect("Field doesn't exist").clone()
}

use std::hash::{Hash, Hasher};
use twox_hash::XxHash64;

pub fn string_to_u64(s: &str) -> u64 {
    let mut hasher = XxHash64::default();
    s.hash(&mut hasher);
    hasher.finish()
}
