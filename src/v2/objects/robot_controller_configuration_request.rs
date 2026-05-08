use crate::v2::objects::network_device::NetworkDevice;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct RobotControllerConfigurationRequest {
    pub network_devices: Vec<NetworkDevice>,

    pub ip: String,
}
