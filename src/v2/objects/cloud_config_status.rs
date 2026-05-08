use crate::v2::objects::cloud_config_status_configured::CloudConfigStatusConfigured;

use crate::v2::objects::cloud_config_status_not_configured::CloudConfigStatusNotConfigured;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub enum CloudConfigStatus {
    CloudConfigStatusConfiguredValue(CloudConfigStatusConfigured),

    CloudConfigStatusNotConfiguredValue(CloudConfigStatusNotConfigured),
}
