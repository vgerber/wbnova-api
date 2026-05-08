use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct DhParameter {
    pub a: Option<f64>,

    pub theta: Option<f64>,

    pub alpha: Option<f64>,

    pub d: Option<f64>,

    pub reverse_rotation_direction: Option<bool>,

    #[serde(alias = "type")]
    pub type_name: Option<String>,
}
