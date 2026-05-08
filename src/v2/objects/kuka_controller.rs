use crate::v2::objects::object::Object;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct KukaController {
    pub rsi_server: Object,

    pub kind: String,

    pub slow_cycle_rate: Option<bool>,

    pub controller_port: i32,

    pub controller_ip: String,
}
