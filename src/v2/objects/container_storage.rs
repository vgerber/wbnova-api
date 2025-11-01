use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ContainerStorage {
    pub mount_path: String,

    pub capacity: String,
}
