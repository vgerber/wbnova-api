use crate::v2::objects::cloud_configuration::CloudConfiguration;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CloudConfigStatusConfigured {
    pub status: String,

    pub config: CloudConfiguration,
}
