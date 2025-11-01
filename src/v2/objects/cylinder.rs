use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct Cylinder {
    pub radius: f64,

    pub shape_type: String,

    pub height: f64,
}
