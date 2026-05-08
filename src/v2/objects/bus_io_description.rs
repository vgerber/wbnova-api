use crate::v2::objects::io_boundary::IoBoundary;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct BusIoDescription {
    pub name: String,

    pub min: Option<IoBoundary>,

    pub io: String,

    pub max: Option<IoBoundary>,

    pub direction: String,

    pub unit: Option<String>,

    pub value_type: String,
}
