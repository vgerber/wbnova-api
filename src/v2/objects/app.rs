use crate::v2::objects::container_storage::ContainerStorage;

use crate::v2::objects::object::Object;

use crate::v2::objects::container_image::ContainerImage;

use crate::v2::objects::container_resources::ContainerResources;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct App {
    pub storage: Option<ContainerStorage>,

    pub app_icon: String,

    pub port: Option<i32>,

    pub diagnosis_path: Option<String>,

    pub environment: Option<Vec<Object>>,

    pub health_path: Option<String>,

    pub name: String,

    pub container_image: ContainerImage,

    pub resources: Option<ContainerResources>,
}
