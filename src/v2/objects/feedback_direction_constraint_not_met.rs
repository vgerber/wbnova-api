use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct FeedbackDirectionConstraintNotMet {
    pub invalid_joint_position: Option<Vec<f64>>,

    pub error_feedback_name: String,
}
