use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct TrajectorySection {
    pub start_location: f64,

    pub end_location: f64,
}
