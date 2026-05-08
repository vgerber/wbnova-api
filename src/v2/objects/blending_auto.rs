use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct BlendingAuto {
    pub blending_name: String,

    pub min_velocity_in_percent: Option<i32>,
}
