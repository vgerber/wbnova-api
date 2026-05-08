use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct LicenseStatus {
    pub status: String,

    pub message: String,
}
