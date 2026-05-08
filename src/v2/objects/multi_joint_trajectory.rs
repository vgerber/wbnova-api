use crate::v2::objects::joint_position_array_dictionary::JointPositionArrayDictionary;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MultiJointTrajectory {
    pub joint_positions_by_motion_group_key: JointPositionArrayDictionary,

    pub locations: Vec<f64>,

    pub times: Vec<f64>,
}
