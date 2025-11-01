use crate::v2::objects::bus_io_profinet::BusIoProfinet;

use crate::v2::objects::modbus_server::ModbusServer;

use crate::v2::objects::modbus_client::ModbusClient;

use crate::v2::objects::bus_io_profinet_virtual::BusIoProfinetVirtual;

use crate::v2::objects::modbus_virtual::ModbusVirtual;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub enum BusIoType {
    BusIoProfinetValue(BusIoProfinet),

    ModbusServerValue(ModbusServer),

    ModbusClientValue(ModbusClient),

    BusIoProfinetVirtualValue(BusIoProfinetVirtual),

    ModbusVirtualValue(ModbusVirtual),
}
