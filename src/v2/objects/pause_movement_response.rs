use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct PauseMovementResponse {
    pub kind: String,

    pub message: Option<String>,
}
