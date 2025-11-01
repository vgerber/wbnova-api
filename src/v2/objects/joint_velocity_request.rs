use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct JointVelocityRequest {
    pub velocity: Vec<f64>,

    pub message_type: String,
}
