use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct JoggingPausedNearCollision {
    pub description: String,

    pub kind: String,
}
