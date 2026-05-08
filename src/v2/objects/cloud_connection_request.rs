use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CloudConnectionRequest {
    pub cloud_hostname: String,

    pub instance_name: String,

    pub token: String,

    pub instance_url: Option<String>,
}
