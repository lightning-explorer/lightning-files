pub mod traits {
    pub mod file_sender_receiver;
    pub mod file_input_into_dir_payload;
    pub mod implementations {
        pub mod tokio_sender_receiver;
    }
}
pub mod db_connected_channel {
    mod receiver;
    mod sender;
    pub mod channel;
}
