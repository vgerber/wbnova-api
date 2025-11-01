use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct VirtualController {
    pub initial_joint_position: Option<String>,

    #[serde(alias = "type")]
    pub type_name: Option<String>,

    pub json: Option<String>,

    pub kind: String,

    pub manufacturer: String,
}
