use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct FeedbackSingularity {
    pub singularity_type: Option<String>,

    pub singular_joint_position: Option<Vec<f64>>,

    pub error_feedback_name: String,
}
