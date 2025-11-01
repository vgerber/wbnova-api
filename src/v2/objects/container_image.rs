use crate::v2::objects::object::Object;

use crate::v2::objects::image_credentials::ImageCredentials;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ContainerImage {
    pub secrets: Option<Vec<Object>>,

    pub image: String,

    pub credentials: Option<ImageCredentials>,
}
