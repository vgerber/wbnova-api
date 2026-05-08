use crate::v2::objects::pose::Pose;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CartesianPtp {
    pub target_pose: Pose,

    pub path_definition_name: String,
}
