use crate::v2::objects::object::Object;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct AbbController {
    pub controller_ip: String,

    pub kind: String,

    pub egm_server: Object,

    pub controller_port: i32,
}
