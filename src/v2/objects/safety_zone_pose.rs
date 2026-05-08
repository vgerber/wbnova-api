use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct SafetyZonePose {
    pub qw: f64,

    pub qy: f64,

    pub x: f64,

    pub qz: f64,

    pub qx: f64,

    pub y: f64,

    pub z: f64,
}
