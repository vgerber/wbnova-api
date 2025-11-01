use crate::v2::objects::external_joint_stream_datapoint::ExternalJointStreamDatapoint;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ExternalJointStreamRequest {
    pub states: Vec<ExternalJointStreamDatapoint>,
}
