use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MotionGroupFromModel {
    pub motion_group_model: String,

    pub initial_joint_position: Option<String>,

    pub motion_group: String,
}
