use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct JoggingPausedNearJointLimit {
    pub kind: String,

    pub joint_indices: Vec<i32>,
}
