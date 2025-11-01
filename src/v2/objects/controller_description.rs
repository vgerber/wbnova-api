use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ControllerDescription {
    pub supports_control: bool,

    pub connected_motion_groups: Vec<String>,

    pub supports_safety_zones: bool,

    pub supports_freedrive: bool,
}
