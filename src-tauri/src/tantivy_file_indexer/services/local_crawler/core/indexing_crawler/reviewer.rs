use std::path::Path;

use crate::tantivy_file_indexer::util::string;

/// Returns `true` if the path is worth getting indexed and processed.
///
/// Example: a cache directory such as C:\rr432j35k321235j5253325 should return false,
/// and the crawler should ignore processing the files inside it and also ignore indexing the directory itself
pub fn path_warrants_processing(path: &Path) -> bool {
    let path_str = path.to_string_lossy().to_string();
    if path_str.len() > 7 && string::calculate_alphabetic_noise_ratio(&path_str) > 0.41 {
        return false;
    }
    true
}
