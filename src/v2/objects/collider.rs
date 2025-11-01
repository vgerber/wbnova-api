use crate::v2::objects::pose::Pose;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct Collider {
    pub margin: Option<f64>,

    pub pose: Option<Pose>,
}
