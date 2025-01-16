use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct CrawlerFile {
    pub path: PathBuf,
    pub priority: u32,
    pub taken: bool,
}
