use crate::v2::objects::image_credentials::ImageCredentials;

use crate::v2::objects::object::Object;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ContainerImage {
    pub image: String,

    pub credentials: Option<ImageCredentials>,

    pub secrets: Option<Vec<Object>>,
}
