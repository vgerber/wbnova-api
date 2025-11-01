use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ListTrajectoriesResponse {
    pub trajectories: Option<Vec<String>>,
}
