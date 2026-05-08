use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct FeedbackCubicSplineIsNotIncreasing {
    pub previous_value: Option<f64>,

    pub error_feedback_name: String,

    pub current_value: Option<f64>,

    pub index: Option<i32>,
}
