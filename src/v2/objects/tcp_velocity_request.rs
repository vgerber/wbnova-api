use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct TcpVelocityRequest {
    pub rotation: Vec<f64>,

    pub use_tool_coordinate_system: Option<bool>,

    pub message_type: String,

    pub translation: Vec<f64>,
}
