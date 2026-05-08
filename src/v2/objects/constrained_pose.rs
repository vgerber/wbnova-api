use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ConstrainedPose {
    pub orientation: f64,

    pub position: Vec<f64>,
}
