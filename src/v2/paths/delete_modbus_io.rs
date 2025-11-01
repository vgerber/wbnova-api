use ::reqwest;

use crate::v2::objects::error::Error;

pub enum DeleteModbusIoResponseType {
    BadRequest(Error),

    UndefinedResponse(reqwest::Response),

    NotFound(Error),
}

pub struct DeleteModbusIoPathParameters {
    pub cell: String,

    pub io: String,
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

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteModbusIoResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(DeleteModbusIoResponseType::UndefinedResponse(response)),
    }
}
