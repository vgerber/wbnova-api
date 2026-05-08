use crate::v2::objects::pose::Pose;

use crate::v2::objects::collision_setups::CollisionSetups;

use crate::v2::objects::limit_range::LimitRange;

use crate::v2::objects::direction_constraint::DirectionConstraint;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ProjectJointPositionDirectionConstraintRequest {
    pub joint_positions: Vec<Vec<f64>>,

    pub mounting: Option<Pose>,

    pub collision_setups: Option<CollisionSetups>,

    pub motion_group_model: String,

    pub joint_position_limits: Option<Vec<LimitRange>>,

    pub tcp_offset: Option<Pose>,

    pub constraint: DirectionConstraint,
}
