use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct RectangularCapsule {
    pub sphere_center_distance_x: f64,

    pub sphere_center_distance_y: f64,

    pub radius: f64,

    pub shape_type: String,
}
