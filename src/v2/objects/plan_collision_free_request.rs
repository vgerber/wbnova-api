use crate::v2::objects::direction_constraint::DirectionConstraint;

use crate::v2::objects::collision_free_algorithm::CollisionFreeAlgorithm;

use crate::v2::objects::motion_group_setup::MotionGroupSetup;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct PlanCollisionFreeRequest {
    pub constraint: Option<DirectionConstraint>,

    pub target: Vec<f64>,

    pub algorithm: CollisionFreeAlgorithm,

    pub motion_group_setup: MotionGroupSetup,

    pub start_joint_position: Vec<f64>,
}
