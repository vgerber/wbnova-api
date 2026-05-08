use crate::v2::objects::tcp_offset_dictionary::TcpOffsetDictionary;

use crate::v2::objects::pose::Pose;

use crate::v2::objects::payload_dictionary::PayloadDictionary;

use crate::v2::objects::collider_dictionary::ColliderDictionary;

use crate::v2::objects::safety_tool_colliders::SafetyToolColliders;

use crate::v2::objects::operation_limits::OperationLimits;

use crate::v2::objects::dh_parameter::DhParameter;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MotionGroupDescription {
    pub tcps: Option<TcpOffsetDictionary>,

    pub flange_offset: Option<Pose>,

    pub serial_number: Option<String>,

    pub motion_group_model: String,

    pub payloads: Option<PayloadDictionary>,

    pub cycle_time: Option<i32>,

    pub kinematic_chain_offset: Option<Pose>,

    pub safety_link_colliders: Option<Vec<ColliderDictionary>>,

    pub safety_zones: Option<ColliderDictionary>,

    pub safety_tool_colliders: Option<SafetyToolColliders>,

    pub operation_limits: OperationLimits,

    pub description_revision: Option<i32>,

    pub mounting: Option<Pose>,

    pub dh_parameters: Option<Vec<DhParameter>>,
}
