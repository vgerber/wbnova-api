use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct PlaybackSpeedRequest {
    pub playback_speed_in_percent: i32,

    pub message_type: String,
}
