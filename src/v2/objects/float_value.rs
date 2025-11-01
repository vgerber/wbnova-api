use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct FloatValue {
    pub value: f64,

    pub value_type: String,
}
