use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ErrorMaxIterationsExceeded {
    pub max_iterations: Option<i32>,

    pub error_feedback_name: String,
}
