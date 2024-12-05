use std::sync::Arc;

use crate::tantivy_file_indexer::services::local_db::table_creator::generate_table_lenient;
use sea_orm::{
    ActiveValue::NotSet, DatabaseConnection, EntityTrait, InsertResult, PaginatorTrait, Set,
};

use super::entities::directory_payload;

#[derive(Clone)]
pub struct IndexerQueueTable {
    db: Arc<DatabaseConnection>,
}

impl IndexerQueueTable {
    pub async fn new_async(db: Arc<DatabaseConnection>) -> Self {
        generate_table_lenient(&db, directory_payload::Entity).await;

        Self { db }
    }

    pub async fn add<T>(
        &self,
        model: T,
    ) -> Result<InsertResult<directory_payload::ActiveModel>, sea_orm::DbErr>
    where
        T: Into<directory_payload::Model>,
    {
        let model = model.into();
        let entry = directory_payload::ActiveModel {
            id: NotSet, // Auto incrementing ID should be set by Sea ORM
            directory_from: Set((model.directory_from).to_owned()),
            files: Set(model.files.to_owned()),
        };
        directory_payload::Entity::insert(entry)
            .exec(&*self.db)
            .await
    }

    pub async fn pop(&self) -> Result<Option<directory_payload::Model>, sea_orm::DbErr> {
        if let Some(entry) = directory_payload::Entity::find().one(&*self.db).await? {
            // Delete the fetched entry:
            _ = directory_payload::Entity::delete_by_id(entry.id)
                .exec(&*self.db)
                .await?;
            return Ok(Some(entry));
        }
        Ok(None)
    }

    pub async fn get_len(&self) -> Result<u64, sea_orm::DbErr> {
        let count = directory_payload::Entity::find().count(&*self.db).await?;
        Ok(count)
    }
}
