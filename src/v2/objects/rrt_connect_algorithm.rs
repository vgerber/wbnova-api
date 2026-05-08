use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct RrtConnectAlgorithm {
    pub apply_smoothing: Option<bool>,

    pub max_step_size: Option<f64>,

    pub adaptive_step_size: Option<bool>,

    pub max_iterations: Option<i32>,

    pub algorithm_name: String,

    pub apply_blending: Option<bool>,
}
