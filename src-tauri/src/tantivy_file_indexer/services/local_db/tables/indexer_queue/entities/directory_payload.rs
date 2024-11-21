use sea_orm::entity::prelude::*;
use serde::Serialize;

use super::indexable_file;

// Corresponds to the FileInputModel struct
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "index_queue")]
#[serde(rename_all = "PascalCase")]
pub struct Model {
    // TODO: consider changing to a different ID in the future, as this one may cause conflicts
    #[sea_orm(primary_key, auto_increment = false)]
    pub directory_from: String,
    pub files: Vec<indexable_file::Model>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    IndexableFile,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::IndexableFile => Entity::belongs_to(super::indexable_file::Entity)
                .from(Column::DirectoryFrom)
                .to(super::indexable_file::Column::DirectoryFrom)
                .into(),
        }
    }
}
impl ActiveModelBehavior for ActiveModel {}
