use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct SafetyGeometryPrism {
    pub bottom: f64,

    pub point: Vec<f64>,

    pub top: f64,
}
