use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct NetworkInterface {
    pub cidr: String,

    pub ip: String,

    pub mac: String,

    pub name: String,
}
