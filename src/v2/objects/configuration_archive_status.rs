use crate::v2::objects::configuration_archive_status_creating::ConfigurationArchiveStatusCreating;

use crate::v2::objects::configuration_archive_status_error::ConfigurationArchiveStatusError;

use crate::v2::objects::configuration_archive_status_success::ConfigurationArchiveStatusSuccess;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub enum ConfigurationArchiveStatus {
    ConfigurationArchiveStatusCreatingValue(ConfigurationArchiveStatusCreating),

    ConfigurationArchiveStatusErrorValue(ConfigurationArchiveStatusError),

    ConfigurationArchiveStatusSuccessValue(ConfigurationArchiveStatusSuccess),
}
