use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MovementErrorResponse {
    pub message: String,

    pub kind: String,
}
