use crate::v2::objects::bus_io_modbus_tcp_server::BusIoModbusTcpServer;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ModbusServer {
    pub input_registers_size: i32,

    pub network: BusIoModbusTcpServer,

    pub discrete_inputs_size: i32,

    pub connections: i32,

    pub bus_type: String,

    pub coils_size: i32,

    pub holding_registers_size: i32,
}
