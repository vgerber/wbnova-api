use crate::v2::objects::collision::Collision;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ErrorJointPositionCollision {
    pub error_feedback_name: String,

    pub joint_position: Option<Vec<f64>>,

    pub collisions: Option<Vec<Collision>>,
}
