use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct TrajectoryRunning {
    pub kind: String,

    pub time_to_end: i32,
}
