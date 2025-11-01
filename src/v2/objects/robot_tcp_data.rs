use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct RobotTcpData {
    pub name: Option<String>,

    pub orientation_type: Option<String>,

    pub orientation: Option<Vec<f64>>,

    pub position: Vec<f64>,
}
