use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ModbusIoData {
    pub description: String,

    #[serde(alias = "type")]
    pub type_name: String,

    pub byte_order: String,

    pub area: String,

    pub address: i32,
}
