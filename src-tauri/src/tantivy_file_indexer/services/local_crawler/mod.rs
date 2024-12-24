pub mod service;
mod core {
    pub mod crawler_queue {
        pub mod queue;
        pub mod filter;
        pub mod trait_impl {
            pub mod crawler_file;
            pub mod crawler_queue_api;
        }
    }
    pub mod indexing_crawler {
        mod crawler;
        mod worker;
        pub mod reviewer;
        pub mod worker_manager;
    }
}
pub mod analyzer {
    pub mod service;
}
pub mod tauri_exports;
