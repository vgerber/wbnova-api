use crate::v2::objects::pause_jogging_response::PauseJoggingResponse;

use crate::v2::objects::joint_velocity_response::JointVelocityResponse;

use crate::v2::objects::movement_error_response::MovementErrorResponse;

use crate::v2::objects::initialize_jogging_response::InitializeJoggingResponse;

use crate::v2::objects::tcp_velocity_response::TcpVelocityResponse;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub enum ExecuteJoggingResponse {
    PauseJoggingResponseValue(PauseJoggingResponse),

    JointVelocityResponseValue(JointVelocityResponse),

    MovementErrorResponseValue(MovementErrorResponse),

    InitializeJoggingResponseValue(InitializeJoggingResponse),

    TcpVelocityResponseValue(TcpVelocityResponse),
}
