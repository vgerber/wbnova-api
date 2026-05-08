use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ConfigurationArchiveStatusCreating {
    pub progress: f64,

    pub status: String,
}
