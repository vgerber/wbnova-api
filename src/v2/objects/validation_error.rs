use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ValidationError {
    #[serde(alias = "type")]
    pub type_name: String,

    pub loc: Vec<i32>,

    pub msg: String,
}
