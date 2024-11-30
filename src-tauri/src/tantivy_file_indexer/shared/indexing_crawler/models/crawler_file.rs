use std::path::PathBuf;

#[derive(Clone)]
pub struct CrawlerFile {
    pub path: PathBuf,
    pub priority: u32,
}
