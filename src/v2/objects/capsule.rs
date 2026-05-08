use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct Capsule {
    pub radius: f64,

    pub shape_type: String,

    pub cylinder_height: f64,
}
