use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CollisionContact {
    pub local: Option<Vec<f64>>,

    pub root: Option<Vec<f64>>,
}
