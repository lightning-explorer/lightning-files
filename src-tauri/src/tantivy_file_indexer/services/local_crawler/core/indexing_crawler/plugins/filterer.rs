use std::path::Path;

use serde::{de::DeserializeOwned, Serialize};

use crate::tantivy_file_indexer::{
    models::auto_serializing_value::AutoSerializingValue,
    services::local_db::tables::app_kv_store::api::AppKvStoreTable, util::string,
};

pub enum ShouldIndexResult{
    ShouldIndex,
    /// The `String` is the reason why it isn't getting indexed
    ShouldNotIndex(String)
}

type JsonVal<T> = AutoSerializingValue<T>;
/// Tells the crawler what to process and what to avoid based on criteria that the user has set
pub struct CrawlerFilterer {
    kv_store: AppKvStoreTable,

    /// If there are ANY values in here, then only files with this extension will get indexed, along with normal directories of course.
    ///
    /// The extensions should not have a leading dot
    whitelisted_extensions: JsonVal<Vec<String>>,
    blacklisted_extensions: JsonVal<Vec<String>>,
    dir_names_exclude: JsonVal<Vec<String>>,
}

impl CrawlerFilterer {
    pub fn new(kv_store: AppKvStoreTable) -> Self {
        Self {
            kv_store,
            whitelisted_extensions: JsonVal::new(Vec::new()),
            blacklisted_extensions: JsonVal::new(Vec::new()),
            dir_names_exclude: JsonVal::new(Vec::new()),
        }
    }

    pub async fn should_crawl_directory(&self, dir_path: &Path) -> bool {
        self.update_json("crawlerDirectoryNamesExclude", &self.dir_names_exclude)
            .await;

        let path_str = dir_path.to_string_lossy().to_string();
        !self
            .dir_names_exclude
            .get_data().await
            .iter()
            .any(|exclude| path_str.contains(exclude))
    }

    pub async fn should_index(&self, path: &Path) -> ShouldIndexResult {
        self.update_json("crawlerWhitelistedExtensions", &self.whitelisted_extensions)
            .await;
        self.update_json("crawlerBlacklistedExtensions", &self.blacklisted_extensions)
            .await;
        let path_str = path.to_string_lossy().to_string();

        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_string();
            let whitelisted_extensions = self.whitelisted_extensions.get_data().await;
            let blacklisted_extensions = self.blacklisted_extensions.get_data().await;

            if !whitelisted_extensions.is_empty() && !whitelisted_extensions.contains(&ext) {
                return ShouldIndexResult::ShouldNotIndex("Extension is not whitelisted".into());
            }
            if blacklisted_extensions.contains(&ext) {
                return ShouldIndexResult::ShouldNotIndex("Extension is blacklisted".into());
            }
        }
        // High noise ratio is the final thing to judge by
        if Self::high_noise_ratio(&path_str){
            return ShouldIndexResult::ShouldNotIndex("High noise ratio".into());
        }
        ShouldIndexResult::ShouldIndex
    }

    /// Returns `true` if the alphabetic noise ratio is high
    ///
    /// Example: a cache directory such as C:\rr432j35k321235j5253325 should return true
    pub fn high_noise_ratio(path: &str) -> bool {
        if path.len() > 7 && string::calculate_alphabetic_noise_ratio(path) > 0.41 {
            return true;
        }
        false
    }

    async fn update_json<T>(&self, key: &str, json: &JsonVal<T>)
    where
        T: Serialize + Clone + DeserializeOwned,
    {
        match self.kv_store.update_value(key, json).await {
            Ok(did_update) => {
                if did_update {
                    println!(
                        "Crawler filter: Noticed update to KV value with key {}",
                        key
                    );
                }
            }
            Err(err) => {
                println!("Crawler Filterer: Error updating {}: {}", key, err);
            }
        }
    }
}
