use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct SafetyGeometryBox {
    pub neighbour: Vec<f64>,

    pub center: Vec<f64>,
}
