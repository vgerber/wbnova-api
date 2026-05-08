use crate::v2::objects::constrained_pose::ConstrainedPose;

use crate::v2::objects::direction_constraint::DirectionConstraint;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct DirectionConstrainedCartesianPtp {
    pub path_definition_name: String,

    pub target_pose: ConstrainedPose,

    pub constraint: DirectionConstraint,
}
