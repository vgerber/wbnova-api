use crate::v2::objects::object::Object;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct InconsistentTrajectorySizeError {
    pub inconsistent_trajectory_size: Option<Object>,
}
