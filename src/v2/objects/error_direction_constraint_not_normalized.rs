use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ErrorDirectionConstraintNotNormalized {
    pub error_feedback_name: String,

    pub direction_constraint: Option<Vec<f64>>,
}
