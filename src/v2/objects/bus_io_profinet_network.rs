use crate::v2::objects::bus_io_profinet_ip_config::BusIoProfinetIpConfig;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct BusIoProfinetNetwork {
    pub device_name: Option<String>,

    pub ip_config: Option<BusIoProfinetIpConfig>,

    pub rema_xml_content: Option<String>,
}
