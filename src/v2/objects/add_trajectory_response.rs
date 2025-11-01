use crate::v2::objects::add_trajectory_error::AddTrajectoryError;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct AddTrajectoryResponse {
    pub error: Option<AddTrajectoryError>,

    pub trajectory: Option<String>,
}
