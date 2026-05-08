use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ImageCredentials {
    pub password: String,

    pub registry: String,

    pub user: String,
}
