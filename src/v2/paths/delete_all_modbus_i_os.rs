use ::reqwest;

use crate::v2::objects::error::Error;

pub enum DeleteAllModbusIOsResponseType {
    BadRequest(Error),

    UndefinedResponse(reqwest::Response),

    NotFound(Error),

    PreconditionFailed(Error),
}

pub struct DeleteAllModbusIOsPathParameters {
    pub cell: String,
}

pub struct DeleteAllModbusIOsQueryParameters {}

pub async fn delete_all_modbus_i_os(
    client: &reqwest::Client,

    server: &str,

    path_parameters: DeleteAllModbusIOsPathParameters,
) -> Result<DeleteAllModbusIOsResponseType, reqwest::Error> {
    let response = match client
        .delete(format!(
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
            Ok(error) => Ok(DeleteAllModbusIOsResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteAllModbusIOsResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        412 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteAllModbusIOsResponseType::PreconditionFailed(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(DeleteAllModbusIOsResponseType::UndefinedResponse(response)),
    }
}
