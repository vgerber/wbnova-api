use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct Plan422Response {
    pub detail: Option<Vec<String>>,
}
