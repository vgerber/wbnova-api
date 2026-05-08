use crate::v2::objects::io_boundary::IoBoundary;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct IoDescription {
    pub io: String,

    pub unit: Option<String>,

    pub value_type: String,

    pub max: Option<IoBoundary>,

    pub group: Option<String>,

    pub direction: String,

    pub name: String,

    pub min: Option<IoBoundary>,
}
