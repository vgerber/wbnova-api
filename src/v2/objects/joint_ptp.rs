use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct JointPtp {
    pub target_joint_position: Vec<f64>,

    pub path_definition_name: String,
}
