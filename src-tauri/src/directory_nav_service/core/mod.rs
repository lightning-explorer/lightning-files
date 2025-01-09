pub mod files {
    pub mod inline_search {
        pub mod tauri_cmds;
    }
    pub mod operations {
        mod cmd_prompt;
        mod common;
        mod file_reader;
        mod metadata;
        pub mod tauri_cmds;
    }
    pub mod retrieve {
        mod file_retriever;
        mod file_sorter;
        mod helper;
        pub mod tauri_cmds;
    }
    pub mod user_input_validator {
        pub mod tauri_cmds;
    }
}
pub mod drives {
    pub mod tauri_cmds;
    mod get_sys_drives;
}
pub mod sys {
    mod info;
    pub mod tauri_cmds;
}
