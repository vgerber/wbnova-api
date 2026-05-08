use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct Cylinder {
    pub height: f64,

    pub radius: f64,

    pub shape_type: String,
}
