use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct FeedbackInvalidSamplingTime {
    pub sampling_time: Option<f64>,

    pub error_feedback_name: String,
}
