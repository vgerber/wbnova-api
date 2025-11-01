use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct FanucController {
    pub kind: String,

    pub controller_ip: String,
}
