use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct LimitRange {
    pub upper_limit: Option<f64>,

    pub lower_limit: Option<f64>,
}
