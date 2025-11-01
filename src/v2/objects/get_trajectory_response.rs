use crate::v2::objects::joint_trajectory::JointTrajectory;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct GetTrajectoryResponse {
    pub tcp: String,

    pub motion_group: String,

    pub trajectory: JointTrajectory,
}
