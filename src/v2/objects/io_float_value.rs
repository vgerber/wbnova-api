use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct IoFloatValue {
    pub value_type: String,

    pub io: String,

    pub value: f64,
}
