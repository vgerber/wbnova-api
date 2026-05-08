use crate::v2::objects::joint_trajectory::JointTrajectory;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct GetTrajectoryResponse {
    pub trajectory: JointTrajectory,

    pub motion_group: String,

    pub tcp: String,
}
