use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ActivateLicenseRequest {
    pub owner_refresh_token: String,
}
