use crate::v2::objects::limits_override::LimitsOverride;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MotionCommand {
    pub limits_override: Option<LimitsOverride>,
}
