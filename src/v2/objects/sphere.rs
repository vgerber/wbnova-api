use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct Sphere {
    pub radius: f64,

    pub shape_type: String,
}
