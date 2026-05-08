use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct InitializeMovementRequest {
    pub initial_location: Option<f64>,

    pub response_coordinate_system: Option<String>,

    pub message_type: String,
}
