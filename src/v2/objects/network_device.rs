use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct NetworkDevice {
    pub vendor: Option<String>,

    pub ip: String,

    pub mac: String,
}
