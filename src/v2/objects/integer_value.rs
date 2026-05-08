use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct IntegerValue {
    pub value_type: String,

    pub value: String,
}
