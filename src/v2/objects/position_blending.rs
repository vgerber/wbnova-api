use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct PositionBlending {
    pub blending_name: String,

    pub position_zone_radius: Option<f64>,
}
