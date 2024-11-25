pub mod service;
mod core {
    pub mod crawler_queue;
    pub mod crawler_worker_manager;
    mod crawler_worker;
}
pub mod analyzer {
    pub mod service;
}
pub mod tauri_exports;
