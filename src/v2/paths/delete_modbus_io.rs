use ::reqwest;

use crate::v2::objects::error::Error;

pub enum DeleteModbusIoResponseType {
    PreconditionFailed(Error),

    BadRequest(Error),

    NotFound(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct DeleteModbusIoPathParameters {
    pub io: String,

    pub cell: String,
}

pub struct DeleteModbusIoQueryParameters {}

pub async fn delete_modbus_io(
    client: &reqwest::Client,

    server: &str,

    path_parameters: DeleteModbusIoPathParameters,
) -> Result<DeleteModbusIoResponseType, reqwest::Error> {
    let response = match client
        .delete(format!(
            "{}/cells/{}/bus-ios/modbus/ios/{}",
            server, path_parameters.cell, path_parameters.io
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        400 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteModbusIoResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        412 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteModbusIoResponseType::PreconditionFailed(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteModbusIoResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(DeleteModbusIoResponseType::UndefinedResponse(response)),
    }
}
