use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CartesianLimits {
    pub velocity: Option<f64>,

    pub orientation_acceleration: Option<f64>,

    pub acceleration: Option<f64>,

    pub jerk: Option<f64>,

    pub orientation_jerk: Option<f64>,

    pub orientation_velocity: Option<f64>,
}
