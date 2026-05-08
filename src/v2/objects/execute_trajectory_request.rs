use crate::v2::objects::start_movement_request::StartMovementRequest;

use crate::v2::objects::playback_speed_request::PlaybackSpeedRequest;

use crate::v2::objects::initialize_movement_request::InitializeMovementRequest;

use crate::v2::objects::pause_movement_request::PauseMovementRequest;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub enum ExecuteTrajectoryRequest {
    StartMovementRequestValue(StartMovementRequest),

    PlaybackSpeedRequestValue(PlaybackSpeedRequest),

    InitializeMovementRequestValue(InitializeMovementRequest),

    PauseMovementRequestValue(PauseMovementRequest),
}
