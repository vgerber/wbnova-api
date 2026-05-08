use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct FeedbackStartJointsMissing {
    pub motion_group_dof: Option<i32>,

    pub start_joints_dof: Option<i32>,

    pub error_feedback_name: String,
}
