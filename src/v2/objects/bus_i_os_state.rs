use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct BusIOsState {
    pub state: String,

    pub message: Option<String>,
}
