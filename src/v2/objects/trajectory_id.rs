use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct TrajectoryId {
    pub message_type: String,

    pub id: String,
}
