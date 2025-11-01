use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct LimitsOverride {
    pub tcp_orientation_velocity_limit: Option<f64>,

    pub joint_acceleration_limits: Option<Vec<f64>>,

    pub tcp_velocity_limit: Option<f64>,

    pub tcp_acceleration_limit: Option<f64>,

    pub joint_velocity_limits: Option<Vec<f64>>,

    pub tcp_orientation_acceleration_limit: Option<f64>,
}
