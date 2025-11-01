use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct BusIoProfinetDefaultRoute {
    pub interface: String,

    pub gateway: String,
}
