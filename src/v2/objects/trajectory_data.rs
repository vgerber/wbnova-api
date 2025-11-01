use crate::v2::objects::joint_trajectory::JointTrajectory;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct TrajectoryData {
    pub data: JointTrajectory,

    pub tcp: Option<String>,

    pub motion_group: Option<String>,

    pub message_type: String,
}
