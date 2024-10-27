use std::fs;
use std::io;
use std::io::Error;
use std::path::Path;
use std::path::PathBuf;

use super::dtos::file_dto::FileDTO;

fn get(directory: &str) -> Result<Vec<FileDTO>, Error> {
    let mut results: Vec<FileDTO> = Vec::new();
    let path = Path::new(directory);

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Ok(dto) = file_dto_from_file(&path) {
                    results.push(dto);
                }
            } else if path.is_dir() {
                println!("Directory: {:?}", path);
            }
        }
    }

    Ok(results)
}

fn file_dto_from_file(file_path: &PathBuf) -> Result<FileDTO, &'static str> {
    Path::new(file_path)
        .file_stem()
        .map(|file_name| FileDTO {
            name: file_name.to_string_lossy().to_string(),
            file_path: file_path.to_string_lossy().to_string(),
            metadata: "".to_string(),
            date_modified: "".to_string(),
            score: 0.0,
        })
        .ok_or("Path is not valid")
}
