use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct BooleanValue {
    pub value: bool,

    pub value_type: String,
}
