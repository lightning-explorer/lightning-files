use std::collections::HashMap;
use std::path::{Path, Component};

use crate::tantivy_file_indexer::models::tantivy_file_model::TantivyFileModel;

type Drive = String;
type Ext = String;

pub fn group_files(paths: Vec<TantivyFileModel>) -> HashMap<(Drive, Ext), Vec<TantivyFileModel>> {
    let mut map: HashMap<(Drive, Ext), Vec<TantivyFileModel>> = HashMap::new();

    for file in paths {
        let path = &file.file_path;
        let drive = Path::new(&path)
            .components()
            .next()
            .and_then(|comp| match comp {
                Component::Prefix(prefix) => Some(prefix.as_os_str().to_string_lossy().into_owned()),
                _ => None,
            })
            .unwrap_or_else(|| "".to_string());

        let ext = Path::new(&path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_string();

        map.entry((drive, ext))
            .or_insert_with(Vec::new)
            .push(file);
    }

    map
}