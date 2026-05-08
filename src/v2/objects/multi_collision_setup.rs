use crate::v2::objects::collider_dictionary::ColliderDictionary;

use crate::v2::objects::collision_motion_group_dictionary::CollisionMotionGroupDictionary;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MultiCollisionSetup {
    pub colliders: Option<ColliderDictionary>,

    pub collision_motion_groups_by_motion_group_key: Option<CollisionMotionGroupDictionary>,
}
