use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CloudRegistrationSuccessResponse {
    pub instance: String,
}
