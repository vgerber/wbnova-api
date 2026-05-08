use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ProfinetIoData {
    pub bit_address: Option<i32>,

    pub description: String,

    pub direction: String,

    pub byte_address: i32,

    #[serde(alias = "type")]
    pub type_name: String,
}
