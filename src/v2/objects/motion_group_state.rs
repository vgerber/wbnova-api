use crate::v2::objects::pose::Pose;

use crate::v2::objects::execute::Execute;

use crate::v2::objects::motion_group_state_joint_limit_reached::MotionGroupStateJointLimitReached;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MotionGroupState {
    pub motion_group: String,

    pub flange_pose: Option<Pose>,

    pub standstill: bool,

    pub payload: Option<String>,

    pub coordinate_system: Option<String>,

    pub joint_current: Option<Vec<f64>>,

    pub timestamp: String,

    pub joint_torque: Option<Vec<f64>>,

    pub controller: String,

    pub tcp_pose: Option<Pose>,

    pub execute: Option<Execute>,

    pub tcp: Option<String>,

    pub joint_limit_reached: MotionGroupStateJointLimitReached,

    pub joint_position: Vec<f64>,

    pub sequence_number: i32,
}
