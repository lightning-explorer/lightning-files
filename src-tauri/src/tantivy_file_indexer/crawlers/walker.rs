use std::fs::{self, DirEntry, ReadDir};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;
use std::vec::Vec;

use crate::tantivy_file_indexer::dtos::file_dto_input::FileDTOInput;
use crate::tantivy_file_indexer::util::file_id_helper;

type DirectoryPath = PathBuf;
/// A recursive file crawler that implements `Iterator`, returning an iterator over each directory.
pub struct FileWalker {
    stack: Vec<(PathBuf, ReadDir)>, // Stack of directory paths and their iterators
}

impl FileWalker {
    /// Creates a new `FileWalker` starting at the given path.
    pub fn new(start_path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let start_path = start_path.as_ref().to_path_buf();
        let start_dir = fs::read_dir(&start_path)?;
        Ok(FileWalker {
            stack: vec![(start_path, start_dir)],
        })
    }

    pub fn new_from_queue(paths:Vec<String>) -> Self{
        Self { stack: get_dirs_with_readers(paths) }
    }

    pub fn get_current_queue(&self) -> Vec<String> {
        self.stack
            .iter()
            .map(|(x, _)| x.to_string_lossy().to_string())
            .collect()
    }
}

impl Iterator for FileWalker {
    type Item = (DirectoryPath, Vec<FileDTOInput>);

    /// Returns an iterator over the next directory's entries.
    fn next(&mut self) -> Option<Self::Item> {
        // Loop until we find a directory with entries or the stack is empty
        while let Some((path, dir)) = self.stack.pop() {
            let mut entries = Vec::new();
            for entry in dir {
                match entry {
                    Ok(entry) => {
                        // Push subdirectories onto the stack to recurse into them
                        if entry.path().is_dir() {
                            if let Ok(sub_dir) = fs::read_dir(entry.path()) {
                                self.stack.push((entry.path(), sub_dir));
                            }
                        }
                        entries.push(Ok(entry));
                    }
                    Err(e) => entries.push(Err(e)), // Capture errors for this directory's entries
                }
            }
            let result: Vec<FileDTOInput> = entries
                .into_iter()
                .filter_map(|x| x.and_then(|y| Ok(create_dto(&y))).ok())
                .filter_map(|x| x.ok())
                .collect();
            return Some((path, result));
        }
        None // No more directories to process
    }
}

fn create_dto(entry: &DirEntry) -> Result<FileDTOInput, String> {
    let metadata = entry.metadata().map_err(|x| x.to_string())?;

    let modified_time = metadata.modified().map_err(|x| x.to_string())?;

    let unix_timestamp = modified_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let file_id = if entry.path().is_dir() {
        //for directories, use the directory path since getting their ID is more difficult
        entry.path().to_string_lossy().to_string()
    } else {
        file_id_helper::get_file_id(entry.path().to_path_buf())?
    };

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

fn get_dirs_with_readers(paths: Vec<String>) -> Vec<(PathBuf, ReadDir)> {
    paths.into_iter()
        .filter_map(|path_str| {
            let path = PathBuf::from(&path_str);
            match fs::read_dir(&path) {
                Ok(read_dir) => Some((path, read_dir)),
                Err(_) => None, // Ignore paths that cannot be read
            }
        })
        .collect()
}