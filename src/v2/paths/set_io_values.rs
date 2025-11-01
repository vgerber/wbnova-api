use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::io_value::IoValue;

pub enum SetIoValuesResponseType {
    NotImplemented(Error),

    BadRequest(Error),

    NotFound(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct SetIoValuesPathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct SetIoValuesQueryParameters {}

pub async fn set_io_values(
    client: &reqwest::Client,

    server: &str,

    content: Vec<IoValue>,

    path_parameters: SetIoValuesPathParameters,
) -> Result<SetIoValuesResponseType, reqwest::Error> {
    let response = match client
        .put(format!(
            "{}/cells/{}/virtual-controllers/{}/ios/values",
            server, path_parameters.cell, path_parameters.controller
        ))
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        501 => match response.json::<Error>().await {
            Ok(error) => Ok(SetIoValuesResponseType::NotImplemented(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(SetIoValuesResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(SetIoValuesResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(SetIoValuesResponseType::UndefinedResponse(response)),
    }
}
