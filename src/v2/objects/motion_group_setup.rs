use crate::v2::objects::collision_setups::CollisionSetups;

use crate::v2::objects::payload::Payload;

use crate::v2::objects::pose::Pose;

use crate::v2::objects::limit_set::LimitSet;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MotionGroupSetup {
    pub collision_setups: Option<CollisionSetups>,

    pub payload: Option<Payload>,

    pub tcp_offset: Option<Pose>,

    pub global_limits: Option<LimitSet>,

    pub cycle_time: i32,

    pub mounting: Option<Pose>,

    pub motion_group_model: String,
}
