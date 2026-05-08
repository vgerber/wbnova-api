use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct UniversalrobotsController {
    pub controller_ip: String,

    pub kind: String,
}
