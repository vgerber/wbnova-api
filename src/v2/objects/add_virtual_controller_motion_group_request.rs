use crate::v2::objects::motion_group_from_json::MotionGroupFromJson;

use crate::v2::objects::motion_group_from_model::MotionGroupFromModel;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub enum AddVirtualControllerMotionGroupRequest {
    MotionGroupFromJsonValue(MotionGroupFromJson),

    MotionGroupFromModelValue(MotionGroupFromModel),
}
