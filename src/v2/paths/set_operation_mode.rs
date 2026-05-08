use ::reqwest;

use crate::v2::objects::error::Error;

pub enum SetOperationModeResponseType {
    BadRequest(Error),

    UndefinedResponse(reqwest::Response),

    NotFound(Error),
}

pub struct SetOperationModePathParameters {
    pub controller: String,

    pub cell: String,
}

pub struct SetOperationModeQueryParameters {
    pub mode: String,
}

pub async fn set_operation_mode(
    client: &reqwest::Client,

    server: &str,

    path_parameters: SetOperationModePathParameters,

    query_parameters: SetOperationModeQueryParameters,
) -> Result<SetOperationModeResponseType, reqwest::Error> {
    // Required Query Parameters
    let reqwest_query_parameters: Vec<(&str, String)> =
        vec![("mode", query_parameters.mode.to_string())];

    let response = match client
        .put(format!(
            "{}/cells/{}/virtual-controllers/{}/operationmode",
            server, path_parameters.cell, path_parameters.controller
        ))
        .query(&reqwest_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        400 => match response.json::<Error>().await {
            Ok(error) => Ok(SetOperationModeResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(SetOperationModeResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(SetOperationModeResponseType::UndefinedResponse(response)),
    }
}
