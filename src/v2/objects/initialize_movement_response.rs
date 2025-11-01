use crate::v2::objects::add_trajectory_error::AddTrajectoryError;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct InitializeMovementResponse {
    pub kind: String,

    pub message: Option<String>,

    pub add_trajectory_error: Option<AddTrajectoryError>,
}
