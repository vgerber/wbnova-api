use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct Error {
    pub message: String,

    pub code: Option<String>,
}
