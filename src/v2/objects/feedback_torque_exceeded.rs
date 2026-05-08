use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct FeedbackTorqueExceeded {
    pub error_feedback_name: String,

    pub torque_value: Option<f64>,

    pub torque_limit: Option<f64>,
}
