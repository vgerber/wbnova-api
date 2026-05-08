use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::flag::Flag;

pub enum GetEmergencyStopResponseType {
    NotFound(Error),

    Ok(Flag),

    UndefinedResponse(reqwest::Response),

    BadRequest(Error),
}

pub struct GetEmergencyStopPathParameters {
    pub controller: String,

    pub cell: String,
}

pub struct GetEmergencyStopQueryParameters {}

pub async fn get_emergency_stop(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetEmergencyStopPathParameters,
) -> Result<GetEmergencyStopResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/virtual-controllers/{}/emergency-stop",
            server, path_parameters.cell, path_parameters.controller
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetEmergencyStopResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(GetEmergencyStopResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<Flag>().await {
            Ok(flag) => Ok(GetEmergencyStopResponseType::Ok(flag)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetEmergencyStopResponseType::UndefinedResponse(response)),
    }
}
