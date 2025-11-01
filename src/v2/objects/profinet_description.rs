use crate::v2::objects::bus_io_profinet_ip_config::BusIoProfinetIpConfig;

use crate::v2::objects::profinet_slot_description::ProfinetSlotDescription;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ProfinetDescription {
    pub ip_config: Option<BusIoProfinetIpConfig>,

    pub device_name: Option<String>,

    pub device_id: String,

    pub slots: Option<Vec<ProfinetSlotDescription>>,

    pub vendor_id: String,
}
