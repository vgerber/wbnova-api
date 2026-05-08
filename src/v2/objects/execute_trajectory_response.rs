use crate::v2::objects::start_movement_response::StartMovementResponse;

use crate::v2::objects::playback_speed_response::PlaybackSpeedResponse;

use crate::v2::objects::movement_error_response::MovementErrorResponse;

use crate::v2::objects::initialize_movement_response::InitializeMovementResponse;

use crate::v2::objects::pause_movement_response::PauseMovementResponse;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub enum ExecuteTrajectoryResponse {
    StartMovementResponseValue(StartMovementResponse),

    PlaybackSpeedResponseValue(PlaybackSpeedResponse),

    MovementErrorResponseValue(MovementErrorResponse),

    InitializeMovementResponseValue(InitializeMovementResponse),

    PauseMovementResponseValue(PauseMovementResponse),
}
