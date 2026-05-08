use crate::v2::objects::object::Object;

use crate::v2::objects::license_status::LicenseStatus;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct License {
    pub feature_flags: Option<Vec<String>>,

    pub owner_email: String,

    pub feature_limitations: Option<Object>,

    pub grace_period_expiry_date: String,

    pub license_expiry_date: Option<String>,

    pub product_name: String,

    pub license_key: String,

    pub consumed_activations: i32,

    pub allowed_activations: i32,

    pub status: LicenseStatus,
}
