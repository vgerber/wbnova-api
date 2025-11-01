use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ImageCredentials {
    pub user: String,

    pub password: String,

    pub registry: String,
}
