use crate::v2::objects::io_boundary::IoBoundary;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct BusIoDescription {
    pub unit: Option<String>,

    pub direction: String,

    pub io: String,

    pub max: Option<IoBoundary>,

    pub name: String,

    pub value_type: String,

    pub min: Option<IoBoundary>,
}
