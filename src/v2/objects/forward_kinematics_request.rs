use crate::v2::objects::pose::Pose;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ForwardKinematicsRequest {
    pub motion_group_model: String,

    pub tcp_offset: Option<Pose>,

    pub joint_positions: Vec<Vec<f64>>,

    pub mounting: Option<Pose>,
}
