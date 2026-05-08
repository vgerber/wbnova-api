use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct GenericBox {
    pub size_y: f64,

    pub box_type: String,

    pub size_z: f64,

    pub size_x: f64,

    pub shape_type: String,
}
