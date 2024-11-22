mod core {
    pub mod index_worker;
    pub mod querier;
}
pub mod models {
    pub mod index_worker {
        pub mod file_input;
    }
}
pub mod traits{
    pub mod file_sender_receiver;
    pub mod implementation{
        pub mod tokio_sender_receiver;
    }
}
pub mod service;
pub mod tauri_exports;