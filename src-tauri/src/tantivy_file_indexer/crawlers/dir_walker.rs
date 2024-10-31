use std::{path::PathBuf, time::UNIX_EPOCH};
use walkdir::{DirEntry, WalkDir};

use crate::tantivy_file_indexer::{dtos::file_dto_input::FileDTOInput, util::file_id_helper};

pub struct DirWalker {
    entries: walkdir::IntoIter,
}

impl DirWalker {
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        DirWalker {
            entries: WalkDir::new(path.into()).into_iter(),
        }
    }
}

impl Iterator for DirWalker {
    type Item = FileDTOInput;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.entries.next() {

            match entry {
                Ok(entry) => {
                    let dto = match create_dto(&entry){
                        Ok(val)=>Some(val),
                        Err(_)=>None,
                    };
                    return dto;
                },
                Err(_) => continue, // Skip any errors (e.g., permission issues)
            }
        }
        None
    }
}

fn create_dto(entry: &DirEntry) -> Result<FileDTOInput, String> {
    let metadata = entry.metadata().map_err(|x| x.to_string())?;

    let modified_time = metadata.modified().map_err(|x| x.to_string())?;

    let unix_timestamp = modified_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let file_id = file_id_helper::get_file_id(entry.path().to_path_buf())?;

    let dto = FileDTOInput {
        file_id,
        name: entry.file_name().to_string_lossy().to_string(),
        file_path: entry.path().to_string_lossy().to_string(),
        metadata: "test metadata".to_string(),
        date_modified: unix_timestamp,
        popularity: 1.0,
    };
    Ok(dto)
}
