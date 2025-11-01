use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MotionGroupJoints {
    pub accelerations: Option<Vec<f64>>,

    pub positions: Vec<f64>,

    pub torques: Option<Vec<f64>>,

    pub velocities: Option<Vec<f64>>,
}
