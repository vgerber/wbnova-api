use crate::v2::objects::feedback_joint_limit_exceeded::FeedbackJointLimitExceeded;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct JointLimitExceededError {
    pub joint_limit_exceeded: Option<FeedbackJointLimitExceeded>,
}
