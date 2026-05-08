use crate::v2::objects::pose::Pose;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CubicSplineParameter {
    pub pose: Pose,

    pub path_parameter: f64,
}
