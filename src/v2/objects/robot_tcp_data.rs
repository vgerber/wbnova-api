use crate::v2::objects::flange_payload::FlangePayload;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct RobotTcpData {
    pub payload: Option<FlangePayload>,

    pub position: Vec<f64>,

    pub orientation_type: Option<String>,

    pub name: Option<String>,

    pub orientation: Option<Vec<f64>>,
}
