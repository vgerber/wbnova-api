use crate::v2::objects::safety_zone::SafetyZone;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct SafetyZones {
    pub safety_zones: Vec<SafetyZone>,
}
