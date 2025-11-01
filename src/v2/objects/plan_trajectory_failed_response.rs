use crate::v2::objects::joint_trajectory::JointTrajectory;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct PlanTrajectoryFailedResponse {
    pub joint_trajectory: Option<JointTrajectory>,

    pub error_location_on_trajectory: f64,
}
