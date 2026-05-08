use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct SafetyGeometrySphere {
    pub center: Vec<f64>,

    pub radius: f64,
}
