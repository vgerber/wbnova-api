use crate::v2::objects::service_status::ServiceStatus;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ServiceStatusResponse {
    pub operating_state: String,

    pub service_status: Vec<ServiceStatus>,
}
