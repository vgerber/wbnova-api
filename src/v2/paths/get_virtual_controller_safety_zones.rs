use ::reqwest;

use crate::v2::objects::safety_zones::SafetyZones;

use crate::v2::objects::error::Error;

pub enum GetVirtualControllerSafetyZonesResponseType {
    BadRequest(Error),

    NotFound(Error),

    Ok(SafetyZones),

    UndefinedResponse(reqwest::Response),
}

pub struct GetVirtualControllerSafetyZonesPathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct GetVirtualControllerSafetyZonesQueryParameters {}

pub async fn get_virtual_controller_safety_zones(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetVirtualControllerSafetyZonesPathParameters,
) -> Result<GetVirtualControllerSafetyZonesResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/virtual-controllers/{}/safety-zones",
            server, path_parameters.cell, path_parameters.controller
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<SafetyZones>().await {
            Ok(safety_zones) => Ok(GetVirtualControllerSafetyZonesResponseType::Ok(
                safety_zones,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(GetVirtualControllerSafetyZonesResponseType::BadRequest(
                error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetVirtualControllerSafetyZonesResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetVirtualControllerSafetyZonesResponseType::UndefinedResponse(response)),
    }
}
