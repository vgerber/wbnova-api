use crate::v2::objects::bus_io_modbus_tcp_client::BusIoModbusTcpClient;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ModbusClient {
    pub bus_type: String,

    pub network: BusIoModbusTcpClient,
}
