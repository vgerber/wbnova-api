use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CloudDisconnectionStatusDisconnecting {
    pub status: String,
}
