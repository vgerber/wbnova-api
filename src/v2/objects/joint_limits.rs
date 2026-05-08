use crate::v2::objects::limit_range::LimitRange;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct JointLimits {
    pub acceleration: Option<f64>,

    pub torque: Option<f64>,

    pub jerk: Option<f64>,

    pub position: Option<LimitRange>,

    pub velocity: Option<f64>,
}
