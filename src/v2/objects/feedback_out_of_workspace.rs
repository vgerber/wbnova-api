use crate::v2::objects::pose::Pose;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct FeedbackOutOfWorkspace {
    pub error_feedback_name: String,

    pub invalid_tcp_pose: Option<Pose>,
}
