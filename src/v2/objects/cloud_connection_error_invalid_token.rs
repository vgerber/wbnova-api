use crate::v2::objects::object::Object;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CloudConnectionErrorInvalidToken {
    pub message: String,

    pub code: String,

    pub details: Object,
}
