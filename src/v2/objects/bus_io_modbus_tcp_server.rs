use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct BusIoModbusTcpServer {
    pub network_type: Option<String>,

    pub port: i32,
}
