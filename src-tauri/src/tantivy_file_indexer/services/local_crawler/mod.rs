pub mod service;
mod core {
    pub mod crawler_queue{
        pub mod queue;
        pub mod trait_impl{
            pub mod crawler_file;
            pub mod crawler_queue_api;
        }
    }
    pub mod file_crawler {
        mod crawler_worker;
        pub mod crawler_worker_manager;
    }
    pub mod indexing_crawler {
        mod crawler;
        mod indexer;
        mod worker;
        pub mod worker_manager;
    }
}
pub mod analyzer {
    pub mod service;
}
pub mod tauri_exports;
