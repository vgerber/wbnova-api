use crate::v2::objects::collision_setups::CollisionSetups;

use crate::v2::objects::blending_position::BlendingPosition;

use crate::v2::objects::joint_trajectory::JointTrajectory;

use crate::v2::objects::limits_override::LimitsOverride;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MergeTrajectoriesSegment {
    pub collision_setups: Option<CollisionSetups>,

    pub blending: Option<BlendingPosition>,

    pub trajectory: JointTrajectory,

    pub limits_override: Option<LimitsOverride>,
}
