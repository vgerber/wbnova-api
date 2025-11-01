use crate::v2::objects::motion_group_joints::MotionGroupJoints;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ExternalJointStreamDatapoint {
    pub motion_group: String,

    pub value: MotionGroupJoints,
}
