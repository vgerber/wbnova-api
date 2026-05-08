use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct JointTrajectory {
    pub locations: Vec<f64>,

    pub times: Vec<f64>,

    pub joint_positions: Vec<Vec<f64>>,
}
