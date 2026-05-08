use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MergeTrajectoriesError {
    pub error_location_on_trajectory: f64,
}
