use crate::v2::objects::io_value::IoValue;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct StreamIoValuesResponse {
    pub sequence_number: i32,

    pub timestamp: String,

    pub io_values: Vec<IoValue>,
}
