use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct PlaybackSpeedRequest {
    pub message_type: String,

    pub playback_speed_in_percent: i32,
}
