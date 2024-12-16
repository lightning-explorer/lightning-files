pub mod models {
    pub mod file;
    pub mod traits;
}
pub mod pipelines {
    pub mod db_tantivy_pipeline;
    pub mod tantivy_pipeline;
}
mod services {
    pub mod task_manager;
}
mod core {
    pub mod engine;
    pub mod indexer;
}
pub mod service;
pub mod tauri_exports;
