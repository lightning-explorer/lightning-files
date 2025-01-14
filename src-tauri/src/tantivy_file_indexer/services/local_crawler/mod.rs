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
        pub mod plugins{
            pub mod garbage_collector;
            pub mod filterer;
            pub mod throttle;
        }
        mod idle;
        mod crawler;
        mod worker;
        mod worker_task_handle;
        pub mod factory;
        pub mod task_manager;
    }
}
// pub mod analyzer {
//     pub mod service;
// }
pub mod tauri_exports;
