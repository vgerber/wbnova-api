use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct BusIoProfinetSlot {
    pub output_size_bytes: i32,

    pub slot: i32,

    pub input_size_bytes: i32,
}
