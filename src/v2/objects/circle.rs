use crate::v2::objects::pose::Pose;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct Circle {
    pub path_definition_name: String,

    pub via_pose: Pose,

    pub target_pose: Pose,
}
