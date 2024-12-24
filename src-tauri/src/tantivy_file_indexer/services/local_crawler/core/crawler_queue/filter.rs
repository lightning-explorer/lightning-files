use std::path::PathBuf;

use crate::tantivy_file_indexer::{dtos::add_to_crawler_queue::AddToCrawlerQueueDTO, services::local_crawler::core::indexing_crawler::reviewer};

/// Reviews the directories that the user wants to add and removes the ones that are likely worthless. Example: cache directories
pub fn filter_out_directories_to_add(directories:Vec<AddToCrawlerQueueDTO>)->Vec<AddToCrawlerQueueDTO>{
    let directories: Vec<AddToCrawlerQueueDTO> = directories
    .into_iter()
    .filter(|dto| reviewer::path_warrants_processing(&PathBuf::from(dto.dir_path.clone())))
    .collect();

    directories
}