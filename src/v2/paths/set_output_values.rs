use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::io_value::IoValue;

pub enum SetOutputValuesResponseType {
    NotFound(Error),

    BadRequest(Error),

    TooManyRequests(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct SetOutputValuesPathParameters {
    pub controller: String,

    pub cell: String,
}

pub struct SetOutputValuesQueryParameters {}

pub async fn set_output_values(
    client: &reqwest::Client,

    server: &str,

    content: Vec<IoValue>,

    path_parameters: SetOutputValuesPathParameters,
) -> Result<SetOutputValuesResponseType, reqwest::Error> {
    let response = match client
        .put(format!(
            "{}/cells/{}/controllers/{}/ios/values",
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
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(SetOutputValuesResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(SetOutputValuesResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        429 => match response.json::<Error>().await {
            Ok(error) => Ok(SetOutputValuesResponseType::TooManyRequests(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(SetOutputValuesResponseType::UndefinedResponse(response)),
    }
}
