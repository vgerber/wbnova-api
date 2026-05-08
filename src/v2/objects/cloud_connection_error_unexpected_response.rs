use crate::v2::objects::object::Object;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CloudConnectionErrorUnexpectedResponse {
    pub details: Object,

    pub code: String,

    pub message: String,
}
