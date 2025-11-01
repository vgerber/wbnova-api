use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ErrorJointLimitExceeded {
    pub joint_index: Option<i32>,

    pub joint_position: Option<Vec<f64>>,

    pub error_feedback_name: String,
}
