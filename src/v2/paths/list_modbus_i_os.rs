use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::modbus_i_os::ModbusIOs;

pub enum ListModbusIOsResponseType {
    NotFound(Error),

    Ok(ModbusIOs),

    UndefinedResponse(reqwest::Response),
}

pub struct ListModbusIOsPathParameters {
    pub cell: String,
}

pub struct ListModbusIOsQueryParameters {}

pub async fn list_modbus_i_os(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ListModbusIOsPathParameters,
) -> Result<ListModbusIOsResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/bus-ios/modbus/ios",
            server, path_parameters.cell
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(ListModbusIOsResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<ModbusIOs>().await {
            Ok(modbus_i_os) => Ok(ListModbusIOsResponseType::Ok(modbus_i_os)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListModbusIOsResponseType::UndefinedResponse(response)),
    }
}
