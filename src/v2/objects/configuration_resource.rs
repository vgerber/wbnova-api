use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ConfigurationResource {
    pub id: String,

    pub children: Option<Vec<ConfigurationResource>>,

    pub name: String,
}
