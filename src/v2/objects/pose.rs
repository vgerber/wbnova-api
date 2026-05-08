use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct Pose {
    pub orientation: Option<Vec<f64>>,

    pub position: Option<Vec<f64>>,
}
