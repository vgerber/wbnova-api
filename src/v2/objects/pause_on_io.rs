use crate::v2::objects::io_value::IoValue;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct PauseOnIo {
    pub comparator: String,

    pub io_origin: String,

    pub io: IoValue,
}
