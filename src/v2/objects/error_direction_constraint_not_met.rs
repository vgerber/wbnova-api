use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ErrorDirectionConstraintNotMet {
    pub joint_position: Option<Vec<f64>>,

    pub error_feedback_name: String,
}
