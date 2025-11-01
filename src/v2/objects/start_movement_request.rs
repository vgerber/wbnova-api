use crate::v2::objects::pause_on_io::PauseOnIo;

use crate::v2::objects::start_on_io::StartOnIo;

use crate::v2::objects::set_io::SetIo;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct StartMovementRequest {
    pub direction: Option<String>,

    pub pause_on_io: Option<PauseOnIo>,

    pub start_on_io: Option<StartOnIo>,

    pub message_type: String,

    pub target_location: Option<f64>,

    pub set_outputs: Option<Vec<SetIo>>,
}
