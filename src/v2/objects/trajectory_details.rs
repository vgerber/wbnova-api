use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct TrajectoryDetails {
    pub kind: String,

    pub location: f64,

    pub trajectory: String,
}
