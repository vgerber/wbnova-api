use crate::v2::objects::pose::Pose;

use crate::v2::objects::dh_parameter::DhParameter;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct KinematicModel {
    pub flange_offset: Option<Pose>,

    pub inverse_solver: Option<String>,

    pub kinematic_chain_offset: Option<Pose>,

    pub dh_parameters: Option<Vec<DhParameter>>,
}
