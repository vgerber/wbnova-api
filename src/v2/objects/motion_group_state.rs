use crate::v2::objects::execute::Execute;

use crate::v2::objects::pose::Pose;

use crate::v2::objects::motion_group_state_joint_limit_reached::MotionGroupStateJointLimitReached;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MotionGroupState {
    pub execute: Option<Execute>,

    pub standstill: bool,

    pub tcp: Option<String>,

    pub tcp_pose: Option<Pose>,

    pub joint_torque: Option<Vec<f64>>,

    pub timestamp: String,

    pub flange_pose: Option<Pose>,

    pub coordinate_system: Option<String>,

    pub payload: Option<String>,

    pub joint_position: Vec<f64>,

    pub joint_limit_reached: MotionGroupStateJointLimitReached,

    pub sequence_number: i32,

    pub description_revision: i32,

    pub joint_current: Option<Vec<f64>>,

    pub controller: String,

    pub motion_group: String,
}
