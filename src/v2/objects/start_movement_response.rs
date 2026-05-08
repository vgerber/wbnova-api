use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct StartMovementResponse {
    pub message: Option<String>,

    pub kind: String,
}
