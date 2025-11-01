use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct PauseJoggingResponse {
    pub message: Option<String>,

    pub kind: String,
}
