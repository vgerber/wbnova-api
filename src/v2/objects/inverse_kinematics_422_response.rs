use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct InverseKinematics422Response {
    pub detail: Option<Vec<String>>,
}
