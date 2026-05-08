use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct FeedbackDirectionConstraintNoSolutionExists {
    pub error_feedback_name: String,

    pub target_joint_position: Option<Vec<f64>>,
}
