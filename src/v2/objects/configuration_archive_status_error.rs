use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ConfigurationArchiveStatusError {
    pub message: String,

    pub status: String,
}
