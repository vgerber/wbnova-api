use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct IoBooleanValue {
    pub value_type: String,

    pub value: bool,

    pub io: String,
}
