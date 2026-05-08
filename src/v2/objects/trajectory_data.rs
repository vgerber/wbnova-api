use crate::v2::objects::joint_trajectory::JointTrajectory;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct TrajectoryData {
    pub motion_group: Option<String>,

    pub tcp: Option<String>,

    pub message_type: String,

    pub data: JointTrajectory,
}
