use sea_orm::entity::prelude::*;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "files")]
/// Should ideally match the structure of `InternalSystemFileModel`
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub path: String,
    pub parent_path: Option<String>,
    pub date_modified: String,
    pub date_created: String,

}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations")
    }
}
impl ActiveModelBehavior for ActiveModel {}