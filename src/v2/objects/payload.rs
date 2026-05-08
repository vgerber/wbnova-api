use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct Payload {
    pub center_of_mass: Option<Vec<f64>>,

    pub moment_of_inertia: Option<Vec<f64>>,

    pub payload: f64,

    pub name: String,
}
