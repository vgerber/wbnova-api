use crate::v2::objects::validation_error::ValidationError;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct HttpValidationError {
    pub detail: Option<Vec<ValidationError>>,
}
