use crate::v2::objects::safety_zone_pose::SafetyZonePose;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct SafetyGeometryLozenge {
    pub y_dimension: f64,

    pub radius: f64,

    pub center: SafetyZonePose,

    pub x_dimension: f64,
}
