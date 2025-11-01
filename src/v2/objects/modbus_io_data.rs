use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ModbusIoData {
    pub address: i32,

    pub byte_order: String,

    #[serde(alias = "type")]
    pub type_name: String,

    pub description: String,

    pub area: String,
}
