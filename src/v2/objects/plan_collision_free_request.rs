use crate::v2::objects::motion_group_setup::MotionGroupSetup;

use crate::v2::objects::collision_free_algorithm::CollisionFreeAlgorithm;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct PlanCollisionFreeRequest {
    pub motion_group_setup: MotionGroupSetup,

    pub target: Vec<f64>,

    pub algorithm: CollisionFreeAlgorithm,

    pub start_joint_position: Vec<f64>,
}
