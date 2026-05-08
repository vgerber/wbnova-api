use crate::v2::objects::error_joint_limit_exceeded_dictionary::ErrorJointLimitExceededDictionary;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MultiErrorJointLimitExceeded {
    pub joint_limit_exceeded_errors_by_motion_group_key: Option<ErrorJointLimitExceededDictionary>,
}
