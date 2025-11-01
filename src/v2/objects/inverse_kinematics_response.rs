use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct InverseKinematicsResponse {
    pub joints: Vec<Vec<Vec<f64>>>,
}
