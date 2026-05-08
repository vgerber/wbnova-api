use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ErrorUnsupportedOperation {
    pub error_feedback_name: String,
}
