use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ConvexHull {
    pub shape_type: String,

    pub vertices: Vec<Vec<f64>>,
}
