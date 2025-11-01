use crate::v2::objects::payload_dictionary::PayloadDictionary;

use crate::v2::objects::dh_parameter::DhParameter;

use crate::v2::objects::collider_dictionary::ColliderDictionary;

use crate::v2::objects::operation_limits::OperationLimits;

use crate::v2::objects::safety_tool_colliders::SafetyToolColliders;

use crate::v2::objects::tcp_offset_dictionary::TcpOffsetDictionary;

use crate::v2::objects::pose::Pose;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MotionGroupDescription {
    pub payloads: Option<PayloadDictionary>,

    pub dh_parameters: Option<Vec<DhParameter>>,

    pub safety_zones: Option<ColliderDictionary>,

    pub operation_limits: OperationLimits,

    pub motion_group_model: String,

    pub safety_link_colliders: Option<Vec<ColliderDictionary>>,

    pub cycle_time: Option<i32>,

    pub safety_tool_colliders: Option<SafetyToolColliders>,

    pub tcps: Option<TcpOffsetDictionary>,

    pub mounting: Option<Pose>,
}
