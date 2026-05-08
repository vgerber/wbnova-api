use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct JointVelocityRequest {
    pub message_type: String,

    pub velocity: Vec<f64>,
}
