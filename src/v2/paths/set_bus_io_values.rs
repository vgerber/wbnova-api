use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::io_value::IoValue;

pub enum SetBusIoValuesResponseType {
    NotFound(Error),

    BadRequest(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct SetBusIoValuesPathParameters {
    pub cell: String,
}

pub struct SetBusIoValuesQueryParameters {}

pub async fn set_bus_io_values(
    client: &reqwest::Client,

    server: &str,

    content: Vec<IoValue>,

    path_parameters: SetBusIoValuesPathParameters,
) -> Result<SetBusIoValuesResponseType, reqwest::Error> {
    let response = match client
        .put(format!(
            "{}/cells/{}/bus-ios/ios/values",
            server, path_parameters.cell
        ))
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        400 => match response.json::<Error>().await {
            Ok(error) => Ok(SetBusIoValuesResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(SetBusIoValuesResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(SetBusIoValuesResponseType::UndefinedResponse(response)),
    }
}
