use crate::v2::objects::double_array_dictionary::DoubleArrayDictionary;

use crate::v2::objects::collision::Collision;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MultiErrorJointPositionCollision {
    pub joint_positions_by_motion_group_key: Option<DoubleArrayDictionary>,

    pub collisions: Option<Vec<Collision>>,
}
