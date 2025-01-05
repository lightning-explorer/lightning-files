use super::{
    entities::kv_pair,
    subscription::{
        backend::BackendSubscriptionList, tauri_subscription_list::TauriSubscriptionList,
    },
};
use crate::tantivy_file_indexer::services::local_db::table_creator::generate_table_lenient;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use tauri::AppHandle;

#[derive(Clone)]
pub struct AppKvStoreTable {
    db: Arc<DatabaseConnection>,

    tauri_subscriptions: TauriSubscriptionList,
    subscriptions: BackendSubscriptionList,
}

impl AppKvStoreTable {
    pub async fn new_async(db: Arc<DatabaseConnection>, app_handle: AppHandle) -> Self {
        generate_table_lenient(&db, kv_pair::Entity).await;

        Self {
            db,
            tauri_subscriptions: TauriSubscriptionList::new(app_handle),
            subscriptions: BackendSubscriptionList::new(),
        }
    }

    /// Set a value in the key-value store
    pub async fn set<T>(&self, key: String, value: T) -> Result<(), String>
    where
        T: Serialize,
    {
        let value = serde_json::to_value(value).map_err(|err| err.to_string())?;

        // Emit the data to Tauri subscribers, if any:
        self.tauri_subscriptions
            .emit_to_subscribers(&key, &value)
            .await;

        let query = r#"
            INSERT INTO kv (key, value)
            VALUES (?, ?)
            ON CONFLICT(key) DO UPDATE SET
                value = excluded.value
        "#;

        sqlx::query(query)
            .bind(&key)
            .bind(&value)
            .execute(self.db.get_sqlite_connection_pool())
            .await
            .map_err(|err| err.to_string())?;

        // Register this operation in the main subscriptions list:
        self.subscriptions.key_changed(&key, value).await;

        Ok(())
    }

    /// Retrieve the value with the certain key in the store
    async fn get_db(&self, key: &str) -> Result<Option<serde_json::Value>, String> {
        Ok(kv_pair::Entity::find()
            .filter(kv_pair::Column::Key.eq(key))
            .one(&*self.db)
            .await
            .map_err(|err| err.to_string())?
            .map(|model| model.value))
    }

    /// Retrieve the value with the certain key in the store, given that it exists and it is in the format you want it in
    /// 
    /// Not necessarily an expensive operation, as results just get cached for future requests.
    pub async fn get<T>(&self, key: &str) -> Result<Option<T>, String>
    where
        T: DeserializeOwned,
    {
        // First, check and see if the subscription storage has the key:
        let val = match self.subscriptions.get_key_status(key).await {
            Some(value) => {
                // The key exists in temporary storage. Good
                Some((*value).clone())
            }
            None => {
                // The key doesn't exist in the storage, but it might exist in the database
                match self.get_db(key).await? {
                    Some(value) => {
                        // The key exists in the database, but not in the temporary storage,
                        // so update the temporary storage to reflect the database:
                        self.subscriptions.key_changed(key, value.clone()).await;
                        Some(value)
                    }
                    None => {
                        // The key does not exist in the database or the temporary storage
                        None
                    }
                }
            }
        };
        match val {
            Some(val) => {
                let new_value = serde_json::from_value(val).map_err(|err| err.to_string())?;
                Ok(Some(new_value))
            }
            None => Ok(None),
        }
    }

    /// Returns the event identifier
    pub async fn tauri_subscribe_to_key(&self, key: &str) -> String {
        self.tauri_subscriptions.add_subscription(key).await
    }
}
