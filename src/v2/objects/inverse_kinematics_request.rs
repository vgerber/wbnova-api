use crate::v2::objects::pose::Pose;

use crate::v2::objects::collision_setups::CollisionSetups;

use crate::v2::objects::limit_range::LimitRange;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct InverseKinematicsRequest {
    pub tcp_poses: Vec<Pose>,

    pub collision_setups: Option<CollisionSetups>,

    pub joint_position_limits: Option<Vec<LimitRange>>,

    pub mounting: Option<Pose>,

    pub motion_group_model: String,

    pub reference_joint_position: Option<Vec<f64>>,

    pub tcp_offset: Option<Pose>,
}
