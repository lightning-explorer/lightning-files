use sea_orm::entity::prelude::*;
use serde::Serialize;

use crate::tantivy_file_indexer::dtos::file_dto_input::FileDTOInput;

// Corresponds to the FileInputModel struct
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "index_queue")]
#[serde(rename_all = "PascalCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id:i32,
    pub directory_from: String,
    #[sea_orm(column_type = "Json")]
    pub files: serde_json::Value,
}

impl Model {
    pub fn get_files(&self) -> Vec<FileDTOInput> {
        serde_json::from_value(self.files.clone()).unwrap_or_default()
    }

    pub fn set_files(&mut self, files: Vec<FileDTOInput>) {
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
