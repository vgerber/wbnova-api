use crate::v2::objects::bus_io_profinet_network::BusIoProfinetNetwork;

use crate::v2::objects::bus_io_profinet_default_route::BusIoProfinetDefaultRoute;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct BusIoProfinet {
    pub network_config: Option<BusIoProfinetNetwork>,

    pub bus_type: String,

    pub mac: String,

    pub default_route: Option<BusIoProfinetDefaultRoute>,

    pub plc_ip: String,
}
