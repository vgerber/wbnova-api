use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct FeedbackInvalidDof {
    pub motion_group_dof: Option<i32>,

    pub error_feedback_name: String,

    pub joint_position: Option<Vec<f64>>,
}
