use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ContainerResources {
    pub intel_gpu: Option<i32>,

    pub memory_limit: Option<String>,
}
