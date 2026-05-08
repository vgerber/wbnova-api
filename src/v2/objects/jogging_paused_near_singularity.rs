use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct JoggingPausedNearSingularity {
    pub kind: String,

    pub description: String,
}
