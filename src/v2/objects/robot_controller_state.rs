use crate::v2::objects::motion_group_state::MotionGroupState;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct RobotControllerState {
    pub motion_groups: Vec<MotionGroupState>,

    pub velocity_override: Option<i32>,

    pub sequence_number: i32,

    pub last_error: Option<Vec<String>>,

    pub controller: String,

    pub mode: String,

    pub safety_state: String,

    pub timestamp: String,

    pub operation_mode: String,
}
