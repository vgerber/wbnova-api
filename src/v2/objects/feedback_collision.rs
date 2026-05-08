use crate::v2::objects::pose::Pose;

use crate::v2::objects::collision::Collision;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct FeedbackCollision {
    pub error_feedback_name: String,

    pub joint_position: Option<Vec<f64>>,

    pub tcp_pose: Option<Pose>,

    pub collisions: Option<Vec<Collision>>,
}
