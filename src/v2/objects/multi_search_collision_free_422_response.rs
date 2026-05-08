use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MultiSearchCollisionFree422Response {
    pub detail: Option<Vec<String>>,
}
