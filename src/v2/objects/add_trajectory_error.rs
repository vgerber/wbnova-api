use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct AddTrajectoryError {
    pub location: Option<f64>,

    pub message: Option<String>,
}
