use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ErrorMotionGroupKeyMismatch {
    pub invalid_motion_group_keys: Option<Vec<String>>,
}
