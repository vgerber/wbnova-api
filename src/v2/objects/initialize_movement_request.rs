use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct InitializeMovementRequest {
    pub message_type: String,

    pub initial_location: Option<f64>,

    pub response_coordinate_system: Option<String>,
}
