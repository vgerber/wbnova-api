use crate::v2::objects::profinet_sub_slot_description::ProfinetSubSlotDescription;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ProfinetSlotDescription {
    pub number: i32,

    pub subslots: Vec<ProfinetSubSlotDescription>,

    pub api: i32,
}
