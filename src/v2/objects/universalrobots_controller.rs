use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct UniversalrobotsController {
    pub kind: String,

    pub controller_ip: String,
}
