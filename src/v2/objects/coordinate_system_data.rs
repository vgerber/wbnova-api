use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CoordinateSystemData {
    pub name: Option<String>,

    pub orientation_type: Option<String>,

    pub reference_coordinate_system: Option<String>,

    pub position: Option<Vec<f64>>,

    pub orientation: Option<Vec<f64>>,
}
