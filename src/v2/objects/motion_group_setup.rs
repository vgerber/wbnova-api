use crate::v2::objects::pose::Pose;

use crate::v2::objects::collision_setups::CollisionSetups;

use crate::v2::objects::limit_set::LimitSet;

use crate::v2::objects::payload::Payload;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MotionGroupSetup {
    pub tcp_offset: Option<Pose>,

    pub collision_setups: Option<CollisionSetups>,

    pub motion_group_model: String,

    pub cycle_time: i32,

    pub global_limits: Option<LimitSet>,

    pub mounting: Option<Pose>,

    pub payload: Option<Payload>,
}
