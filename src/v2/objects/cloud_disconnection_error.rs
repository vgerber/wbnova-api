use crate::v2::objects::cloud_connection_error_leafnode_restart_timeout::CloudConnectionErrorLeafnodeRestartTimeout;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CloudDisconnectionError {
    pub error: CloudConnectionErrorLeafnodeRestartTimeout,
}
