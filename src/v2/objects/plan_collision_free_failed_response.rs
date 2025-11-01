use crate::v2::objects::error_max_iterations_exceeded::ErrorMaxIterationsExceeded;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct PlanCollisionFreeFailedResponse {
    pub error_feedback: ErrorMaxIterationsExceeded,
}
