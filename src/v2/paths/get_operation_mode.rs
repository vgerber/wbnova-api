use ::reqwest;

use crate::v2::objects::op_mode::OpMode;

use crate::v2::objects::error::Error;

pub enum GetOperationModeResponseType {
    BadRequest(Error),

    UndefinedResponse(reqwest::Response),

    Ok(OpMode),
}

pub struct GetOperationModePathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct GetOperationModeQueryParameters {}

pub async fn get_operation_mode(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetOperationModePathParameters,
) -> Result<GetOperationModeResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/virtual-controllers/{}/operationmode",
            server, path_parameters.cell, path_parameters.controller
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<OpMode>().await {
            Ok(op_mode) => Ok(GetOperationModeResponseType::Ok(op_mode)),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(GetOperationModeResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetOperationModeResponseType::UndefinedResponse(response)),
    }
}
