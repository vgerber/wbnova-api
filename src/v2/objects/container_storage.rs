use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ContainerStorage {
    pub capacity: String,

    pub mount_path: String,
}
