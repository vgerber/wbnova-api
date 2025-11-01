use crate::v2::objects::object::Object;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ServiceStatus {
    pub status: Object,

    pub group: String,

    pub service: String,
}
