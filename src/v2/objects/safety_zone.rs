use crate::v2::objects::safety_geometry::SafetyGeometry;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct SafetyZone {
    pub active: Option<bool>,

    pub geometry: SafetyGeometry,

    pub priority: Option<i32>,

    pub name: String,

    pub restricted: Option<bool>,

    pub id: i32,

    pub uid_ref_cs: Option<String>,

    pub mg_uid: Option<i32>,
}
