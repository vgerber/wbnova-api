use crate::v2::objects::pose::Pose;

use crate::v2::objects::limit_range::LimitRange;

use crate::v2::objects::collision_setups::CollisionSetups;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct InverseKinematicsRequest {
    pub motion_group_model: String,

    pub mounting: Option<Pose>,

    pub tcp_offset: Option<Pose>,

    pub tcp_poses: Vec<Pose>,

    pub joint_position_limits: Option<Vec<LimitRange>>,

    pub collision_setups: Option<CollisionSetups>,
}
