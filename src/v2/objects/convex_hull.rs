use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ConvexHull {
    pub vertices: Vec<Vec<f64>>,

    pub shape_type: String,
}
