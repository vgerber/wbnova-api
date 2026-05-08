use ::reqwest;

use crate::v2::objects::error::Error;

pub enum SetEmergencyStopResponseType {
    NotFound(Error),

    UndefinedResponse(reqwest::Response),

    BadRequest(Error),
}

pub struct SetEmergencyStopPathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct SetEmergencyStopQueryParameters {
    pub active: Option<bool>,
}

pub async fn set_emergency_stop(
    client: &reqwest::Client,

    server: &str,

    path_parameters: SetEmergencyStopPathParameters,

    query_parameters: SetEmergencyStopQueryParameters,
) -> Result<SetEmergencyStopResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.active {
        reqwest_query_parameters.push(("active", query_parameter.to_string()));
    }

    let response = match client
        .put(format!(
            "{}/cells/{}/virtual-controllers/{}/emergency-stop",
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
            Ok(error) => Ok(SetEmergencyStopResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(SetEmergencyStopResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(SetEmergencyStopResponseType::UndefinedResponse(response)),
    }
}
