use crate::v2::objects::direction_constraint::DirectionConstraint;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct DirectionConstrainedJointPtp {
    pub constraint: DirectionConstraint,

    pub target_joint_position: Vec<f64>,

    pub path_definition_name: String,
}
