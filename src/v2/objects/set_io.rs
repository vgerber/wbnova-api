use crate::v2::objects::io_value::IoValue;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct SetIo {
    pub io: IoValue,

    pub location: f64,

    pub io_origin: String,
}
