use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MidpointInsertionAlgorithm {
    pub algorithm_name: String,

    pub max_iterations: Option<i32>,
}
