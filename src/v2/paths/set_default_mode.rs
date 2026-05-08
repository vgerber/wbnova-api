use ::reqwest;

use crate::v2::objects::error::Error;

pub enum SetDefaultModeResponseType {
    NotFound(Error),

    BadRequest(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct SetDefaultModePathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct SetDefaultModeQueryParameters {
    pub mode: String,
}

pub async fn set_default_mode(
    client: &reqwest::Client,

    server: &str,

    path_parameters: SetDefaultModePathParameters,

    query_parameters: SetDefaultModeQueryParameters,
) -> Result<SetDefaultModeResponseType, reqwest::Error> {
    // Required Query Parameters
    let reqwest_query_parameters: Vec<(&str, String)> =
        vec![("mode", query_parameters.mode.to_string())];

    let response = match client
        .put(format!(
            "{}/cells/{}/controllers/{}/mode",
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
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(SetDefaultModeResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(SetDefaultModeResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(SetDefaultModeResponseType::UndefinedResponse(response)),
    }
}
