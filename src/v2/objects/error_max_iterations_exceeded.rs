use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ErrorMaxIterationsExceeded {
    pub error_feedback_name: String,

    pub max_iterations: Option<i32>,
}
