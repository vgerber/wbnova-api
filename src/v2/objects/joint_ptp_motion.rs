use crate::v2::objects::limits_override::LimitsOverride;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct JointPtpMotion {
    pub start_joint_position: Vec<f64>,

    pub limits_override: Option<LimitsOverride>,

    pub target_joint_position: Vec<f64>,
}
