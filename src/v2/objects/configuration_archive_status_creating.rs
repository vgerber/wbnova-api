use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ConfigurationArchiveStatusCreating {
    pub status: String,

    pub progress: f64,
}
