use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct FeedbackDirectionConstraintNotNormalized {
    pub direction_constraint: Option<Vec<f64>>,

    pub error_feedback_name: String,
}
