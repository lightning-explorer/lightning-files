use std::{collections::HashMap, sync::Arc};
use sea_orm::{
    prelude::Expr, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use serde::Serialize;
use sqlx::{Sqlite, Transaction};
use crate::tantivy_file_indexer::services::local_db::table_creator::generate_table_lenient;
use super::entities::kv_pair;

#[derive(Clone)]
pub struct AppKvStore {
    db: Arc<DatabaseConnection>,
}

impl AppKvStore {
    pub async fn new_async(db: Arc<DatabaseConnection>) -> Self {
        generate_table_lenient(&db, kv_pair::Entity).await;

        Self { db }
    }

    pub async fn set<T>(&self, key:String, value:T) -> Result<(), String> where T:Serialize {
        let value = serde_json::to_value(value).map_err(|err|err.to_string())?;
        let model = kv_pair::Model{
            key,
            value
        };
        let mut transaction: Transaction<'_, Sqlite> =
            self.db.get_sqlite_connection_pool().begin().await.map_err(|err|err.to_string())?;

        let query = r#"
            INSERT INTO indexed (key, value)
            VALUES (?, ?)
            ON CONFLICT(key) DO UPDATE SET
                value = excluded.value
        "#;

        sqlx::query(query)
            .bind(&model.key)
            .bind(&model.value)
            .execute(&mut *transaction)
            .await.map_err(|err|err.to_string())?;
        
        transaction.commit().await.map_err(|err|err.to_string())?;
        Ok(())
    }
}