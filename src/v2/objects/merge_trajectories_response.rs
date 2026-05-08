use crate::v2::objects::joint_trajectory::JointTrajectory;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MergeTrajectoriesResponse {
    pub joint_trajectory: Option<JointTrajectory>,
}
