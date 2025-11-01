use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct BusIOsState {
    pub message: Option<String>,

    pub state: String,
}
