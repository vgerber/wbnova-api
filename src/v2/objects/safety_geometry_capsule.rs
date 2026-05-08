use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct SafetyGeometryCapsule {
    pub radius: f64,

    pub top: Vec<f64>,

    pub bottom: Vec<f64>,
}
