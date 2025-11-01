use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ProfinetIoData {
    #[serde(alias = "type")]
    pub type_name: String,

    pub description: String,

    pub byte_address: i32,

    pub bit_address: Option<i32>,

    pub direction: String,
}
