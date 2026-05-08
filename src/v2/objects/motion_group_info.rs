use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MotionGroupInfo {
    pub motion_group: String,

    pub name: String,

    pub dof: i32,
}
