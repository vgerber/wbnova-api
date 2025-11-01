use crate::v2::objects::object::Object;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct KukaController {
    pub controller_port: i32,

    pub slow_cycle_rate: Option<bool>,

    pub controller_ip: String,

    pub rsi_server: Object,

    pub kind: String,
}
