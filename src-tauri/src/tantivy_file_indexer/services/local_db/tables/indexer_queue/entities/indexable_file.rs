use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "indexer_files")]
#[serde(rename_all = "PascalCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    pub directory_from: String,

    pub file_id: String,
    pub name: String,
    pub file_path: String,
    pub metadata: String,
    pub date_modified: u64, // UNIX timestamp
    pub popularity: f64,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations")
    }
}
impl ActiveModelBehavior for ActiveModel {}
