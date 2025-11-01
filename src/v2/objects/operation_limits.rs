use crate::v2::objects::limit_set::LimitSet;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct OperationLimits {
    pub manual_limits: Option<LimitSet>,

    #[serde(alias = "manual_t1_limits")]
    pub manual_t_1_limits: Option<LimitSet>,

    pub auto_limits: Option<LimitSet>,

    #[serde(alias = "manual_t2_limits")]
    pub manual_t_2_limits: Option<LimitSet>,
}
