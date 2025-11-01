use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct VirtualRobotConfiguration {
    pub content: String,

    pub name: String,
}
