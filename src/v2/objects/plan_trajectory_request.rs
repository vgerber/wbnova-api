use crate::v2::objects::motion_command::MotionCommand;

use crate::v2::objects::motion_group_setup::MotionGroupSetup;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct PlanTrajectoryRequest {
    pub start_joint_position: Vec<f64>,

    pub motion_commands: Vec<MotionCommand>,

    pub motion_group_setup: MotionGroupSetup,
}
