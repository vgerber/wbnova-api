use crate::v2::objects::io_boundary::IoBoundary;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct IoDescription {
    pub direction: String,

    pub unit: Option<String>,

    pub max: Option<IoBoundary>,

    pub value_type: String,

    pub group: Option<String>,

    pub io: String,

    pub min: Option<IoBoundary>,

    pub name: String,
}
