use crate::v2::objects::add_trajectory_error::AddTrajectoryError;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct InitializeMovementResponse {
    pub add_trajectory_error: Option<AddTrajectoryError>,

    pub message: Option<String>,

    pub kind: String,
}
