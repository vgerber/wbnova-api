use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct TrajectoryId {
    pub id: String,

    pub message_type: String,
}
