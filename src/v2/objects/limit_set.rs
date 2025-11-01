use crate::v2::objects::joint_limits::JointLimits;

use crate::v2::objects::cartesian_limits::CartesianLimits;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct LimitSet {
    pub joints: Option<Vec<JointLimits>>,

    pub elbow: Option<CartesianLimits>,

    pub tcp: Option<CartesianLimits>,

    pub coupled_shoulder_elbow_joint: Option<JointLimits>,

    pub flange: Option<CartesianLimits>,
}
