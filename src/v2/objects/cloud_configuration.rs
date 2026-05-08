use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CloudConfiguration {
    pub cloud_hostname: String,

    pub instance: String,
}
