use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct FeedbackInvalidNanValue {
    pub error_feedback_name: String,

    pub value: Option<Vec<f64>>,
}
