use std::{fs, path::PathBuf};

use dirs::data_dir;
use serde::{de::DeserializeOwned, Serialize};

use crate::tantivy_file_indexer::services::app_save::core::helper::{
    create_file, load, save,
};

pub enum AppSavePath {
    AppData,
    Other(PathBuf),
}
pub struct AppSaveService {
    pub save_dir: PathBuf,
}

impl AppSaveService {
    pub fn new(save_dir: AppSavePath, app_name: &str) -> Self {
        let save_path = AppSaveService::get_save_path(save_dir, app_name);
        if !save_path.exists() {
            fs::create_dir_all(app_name).expect("could not create App directory");
        }
        Self {
            save_dir: save_path,
        }
    }
    pub fn create_path(&self, path: &str) -> PathBuf {
        create_file(&self.save_dir, path)
    }

    /// Save arbitrary data in JSON format on the disk.
    /// 
    /// Note: do not include '.json' when you pass in a value for `name`
    pub fn save<T>(&self, name: &str, data: T) -> Result<(), std::io::Error>
    where
        T: Serialize,
    {
        save(&self.save_dir, name, data)
    }

    /**
     * Note: do not include '.json' when you pass in a value for `name`
     */
    pub fn load<T>(&self, name: &str) -> Result<T, std::io::Error>
    where
        T: DeserializeOwned,
    {
        load(&self.save_dir, name)
    }

    fn get_save_path(save_dir: AppSavePath, app_name: &str) -> PathBuf {
        let save_path = match save_dir {
            AppSavePath::AppData => data_dir().expect("Could not find AppData directory"),
            AppSavePath::Other(path) => path,
        };
        save_path.join(app_name)
    }
}
