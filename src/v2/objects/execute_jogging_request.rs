use crate::v2::objects::tcp_velocity_request::TcpVelocityRequest;

use crate::v2::objects::pause_jogging_request::PauseJoggingRequest;

use crate::v2::objects::joint_velocity_request::JointVelocityRequest;

use crate::v2::objects::initialize_jogging_request::InitializeJoggingRequest;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub enum ExecuteJoggingRequest {
    TcpVelocityRequestValue(TcpVelocityRequest),

    PauseJoggingRequestValue(PauseJoggingRequest),

    JointVelocityRequestValue(JointVelocityRequest),

    InitializeJoggingRequestValue(InitializeJoggingRequest),
}
