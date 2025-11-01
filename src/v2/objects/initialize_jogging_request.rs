use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct InitializeJoggingRequest {
    pub motion_group: String,

    pub response_coordinate_system: Option<String>,

    pub tcp: Option<String>,

    pub message_type: String,
}
