use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ProfinetInputOutputConfig {
    pub config: String,

    pub input_offset: i32,

    pub output_offset: i32,
}
