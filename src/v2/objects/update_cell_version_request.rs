use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct UpdateCellVersionRequest {
    pub channel: String,
}
