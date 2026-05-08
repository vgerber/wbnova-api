use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct InitializeJoggingRequest {
    pub tcp: Option<String>,

    pub motion_group: String,

    pub message_type: String,
}
