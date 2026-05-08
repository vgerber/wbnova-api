use crate::v2::objects::object::Object;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct TorqueExceededError {
    pub torque_exceeded: Option<Object>,

    pub kind: String,
}
