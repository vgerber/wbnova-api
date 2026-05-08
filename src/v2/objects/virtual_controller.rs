use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct VirtualController {
    #[serde(alias = "type")]
    pub type_name: Option<String>,

    pub json: Option<String>,

    pub kind: String,

    pub manufacturer: String,

    pub initial_joint_position: Option<String>,
}
