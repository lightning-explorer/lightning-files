use super::entities::kv_pair;
use crate::tantivy_file_indexer::services::local_db::table_creator::generate_table_lenient;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{de::DeserializeOwned, Serialize};
use sqlx::{Sqlite, Transaction};
use tauri::AppHandle;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppKvStore {
    db: Arc<DatabaseConnection>,
}

impl AppKvStore {
    pub async fn new_async(db: Arc<DatabaseConnection>, app_handle:AppHandle) -> Self {
        generate_table_lenient(&db, kv_pair::Entity).await;

        Self { db }
    }

    /// Set a value in the key-value store
    pub async fn set<T>(&self, key: String, value: T) -> Result<(), String>
    where
        T: Serialize,
    {
        let value = serde_json::to_value(value).map_err(|err| err.to_string())?;
        let model = kv_pair::Model { key, value };
        let mut transaction: Transaction<'_, Sqlite> = self
            .db
            .get_sqlite_connection_pool()
            .begin()
            .await
            .map_err(|err| err.to_string())?;

        let query = r#"
            INSERT INTO kv (key, value)
            VALUES (?, ?)
            ON CONFLICT(key) DO UPDATE SET
                value = excluded.value
        "#;

        sqlx::query(query)
            .bind(&model.key)
            .bind(&model.value)
            .execute(&mut *transaction)
            .await
            .map_err(|err| err.to_string())?;

        transaction.commit().await.map_err(|err| err.to_string())?;
        Ok(())
    }

    /// Retrieve the value with the certain key in the store, given that it exists and it is in the format you want it in
    pub async fn get<T>(&self, key: String) -> Result<Option<T>, String>
    where
        T: DeserializeOwned,
    {
        let model_maybe = kv_pair::Entity::find()
            .filter(kv_pair::Column::Key.eq(key))
            .one(&*self.db)
            .await
            .map_err(|err| err.to_string())?;

        match model_maybe {
            Some(model) => {
                let deserialized =
                    serde_json::from_value::<T>(model.value).map_err(|err| err.to_string())?;
                Ok(Some(deserialized))
            }
            None => Ok(None),
        }
    }
}
