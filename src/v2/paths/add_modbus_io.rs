use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::modbus_io_data::ModbusIoData;

pub enum AddModbusIoResponseType {
    PreconditionFailed(Error),

    BadRequest(Error),

    UndefinedResponse(reqwest::Response),

    NotFound(Error),
}

pub struct AddModbusIoPathParameters {
    pub io: String,

    pub cell: String,
}

pub struct AddModbusIoQueryParameters {}

pub async fn add_modbus_io(
    client: &reqwest::Client,

    server: &str,

    content: ModbusIoData,

    path_parameters: AddModbusIoPathParameters,
) -> Result<AddModbusIoResponseType, reqwest::Error> {
    let response = match client
        .put(format!(
            "{}/cells/{}/bus-ios/modbus/ios/{}",
            server, path_parameters.cell, path_parameters.io
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
            Ok(error) => Ok(AddModbusIoResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(AddModbusIoResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        412 => match response.json::<Error>().await {
            Ok(error) => Ok(AddModbusIoResponseType::PreconditionFailed(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(AddModbusIoResponseType::UndefinedResponse(response)),
    }
}
