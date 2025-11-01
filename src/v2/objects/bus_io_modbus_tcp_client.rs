use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct BusIoModbusTcpClient {
    pub ip: String,

    pub network_type: Option<String>,

    pub port: i32,
}
