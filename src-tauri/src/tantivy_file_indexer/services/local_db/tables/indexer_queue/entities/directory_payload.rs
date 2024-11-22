use sea_orm::entity::prelude::*;
use serde::Serialize;

use super::indexable_file::{self, IndexableFile};

// Corresponds to the FileInputModel struct
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "index_queue")]
#[serde(rename_all = "PascalCase")]
pub struct Model {
    // TODO: consider changing to a different ID in the future, as this one may cause conflicts
    #[sea_orm(primary_key, auto_increment = false)]
    pub directory_from: String,
    #[sea_orm(column_type = "Json")]
    pub files: serde_json::Value,
}

impl Model {
    pub fn get_files(&self) -> Vec<IndexableFile> {
        serde_json::from_value(self.files.clone()).unwrap_or_default()
    }

    pub fn set_files(&mut self, files: Vec<IndexableFile>) {
        self.files = serde_json::to_value(files).unwrap();
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations")
    }
}
impl ActiveModelBehavior for ActiveModel {}
