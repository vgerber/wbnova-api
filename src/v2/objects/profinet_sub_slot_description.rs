use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ProfinetSubSlotDescription {
    pub output_length: i32,

    pub input_length: i32,

    pub number: i32,
}
