use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct DirectionConstraint {
    pub tcp: Vec<f64>,

    pub tolerance: f64,

    pub constraint_name: String,

    pub world: Vec<f64>,
}
