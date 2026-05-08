use crate::v2::objects::error_invalid_joint_count_dictionary::ErrorInvalidJointCountDictionary;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MultiErrorInvalidJointCount {
    pub invalid_joint_count_errors_by_motion_group_key: Option<ErrorInvalidJointCountDictionary>,
}
