use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct Sphere {
    pub shape_type: String,

    pub radius: f64,
}
