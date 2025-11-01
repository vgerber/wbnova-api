use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct IoIntegerValue {
    pub value: String,

    pub value_type: String,

    pub io: String,
}
