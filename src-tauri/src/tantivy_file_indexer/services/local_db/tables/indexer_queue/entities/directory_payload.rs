use sea_orm::entity::prelude::*;
use serde::Serialize;

use crate::tantivy_file_indexer::models::internal_system_file;

// Corresponds to the FileInputModel struct
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "index_queue")]
#[serde(rename_all = "PascalCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub directory_from: String,
    #[sea_orm(column_type = "Json")]
    pub files: serde_json::Value,
}

impl Model {
    pub fn get_files(&self) -> Vec<internal_system_file::model::Model> {
        serde_json::from_value(self.files.clone()).unwrap_or_default()
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
