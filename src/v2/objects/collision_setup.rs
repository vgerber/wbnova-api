use crate::v2::objects::collider_dictionary::ColliderDictionary;

use crate::v2::objects::collision_motion_group_tool::CollisionMotionGroupTool;

use crate::v2::objects::collision_motion_group_link::CollisionMotionGroupLink;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CollisionSetup {
    pub colliders: Option<ColliderDictionary>,

    pub self_collision_detection: Option<bool>,

    pub tool: Option<CollisionMotionGroupTool>,

    pub link_chain: Option<Vec<CollisionMotionGroupLink>>,
}
