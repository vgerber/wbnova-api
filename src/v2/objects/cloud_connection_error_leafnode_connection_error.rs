use crate::v2::objects::object::Object;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CloudConnectionErrorLeafnodeConnectionError {
    pub message: String,

    pub code: String,

    pub details: Object,
}
