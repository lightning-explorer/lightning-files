pub mod service;
pub mod traits{
    pub mod crawler_queue_api;
    pub mod files_collection_api;
}
pub mod models{
    pub mod crawler_file;
    pub mod file_model;
}
mod core {
    pub mod crawler_queue;
    pub mod file_crawler {
        mod crawler_worker;
        pub mod crawler_worker_manager;
    }
    pub mod indexing_crawler{
        pub mod worker_manager;
        mod indexer;
    }
}
pub mod analyzer {
    pub mod service;
}
pub mod tauri_exports;
