use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct BusIoProfinetIpConfig {
    pub netmask: String,

    pub gateway: String,

    pub ip: String,
}
