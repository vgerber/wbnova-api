use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MotionGroupStateJointLimitReached {
    pub limit_reached: Vec<bool>,
}
