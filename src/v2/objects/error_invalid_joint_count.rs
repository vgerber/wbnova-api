use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ErrorInvalidJointCount {
    pub expected_joint_count: i32,

    pub provided_joint_count: i32,

    pub error_feedback_name: String,
}
