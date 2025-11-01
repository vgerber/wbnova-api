use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MotionGroupInfo {
    pub name: String,

    pub dof: i32,

    pub motion_group: String,
}
