use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetFilesParamsModel {
    pub show_hidden: bool,
}
