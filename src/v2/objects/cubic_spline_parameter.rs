use crate::v2::objects::pose::Pose;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CubicSplineParameter {
    pub path_parameter: f64,

    pub pose: Pose,
}
