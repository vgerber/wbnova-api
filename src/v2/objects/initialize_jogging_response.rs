use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct InitializeJoggingResponse {
    pub kind: String,

    pub message: Option<String>,
}
