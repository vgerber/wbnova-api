use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CloudConnectionErrorLeafnodeRestartTimeout {
    pub message: String,

    pub code: String,
}
