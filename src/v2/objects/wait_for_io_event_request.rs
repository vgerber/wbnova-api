use crate::v2::objects::io_value::IoValue;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct WaitForIoEventRequest {
    pub io: IoValue,

    pub comparator: String,
}
