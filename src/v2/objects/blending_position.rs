use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct BlendingPosition {
    pub orientation_zone_percentage: Option<f64>,

    pub space: Option<String>,

    pub position_zone_radius: Option<f64>,

    pub joints_zone_radius: Option<f64>,

    pub position_zone_percentage: Option<f64>,

    pub blending_name: String,

    pub joints_zone_percentage: Option<f64>,

    pub orientation_zone_radius: Option<f64>,
}
