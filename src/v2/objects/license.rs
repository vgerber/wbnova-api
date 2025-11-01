use crate::v2::objects::object::Object;

use crate::v2::objects::license_status::LicenseStatus;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct License {
    pub allowed_activations: i32,

    pub owner_email: String,

    pub feature_limitations: Option<Object>,

    pub product_name: String,

    pub status: LicenseStatus,

    pub consumed_activations: i32,

    pub license_key: String,

    pub feature_flags: Option<Vec<String>>,

    pub license_expiry_date: Option<String>,

    pub grace_period_expiry_date: String,
}
