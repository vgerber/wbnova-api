use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct JointVelocityResponse {
    pub message: Option<String>,

    pub kind: String,
}
