use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::safety_zone::SafetyZone;

pub enum AddVirtualControllerSafetyZoneResponseType {
    UndefinedResponse(reqwest::Response),

    BadRequest(Error),

    NotFound(Error),
}

pub struct AddVirtualControllerSafetyZonePathParameters {
    pub controller: String,

    pub cell: String,
}

pub struct AddVirtualControllerSafetyZoneQueryParameters {}

pub async fn add_virtual_controller_safety_zone(
    client: &reqwest::Client,

    server: &str,

    content: SafetyZone,

    path_parameters: AddVirtualControllerSafetyZonePathParameters,
) -> Result<AddVirtualControllerSafetyZoneResponseType, reqwest::Error> {
    let response = match client
        .post(format!(
            "{}/cells/{}/virtual-controllers/{}/safety-zones",
            server, path_parameters.cell, path_parameters.controller
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
            Ok(error) => Ok(AddVirtualControllerSafetyZoneResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(AddVirtualControllerSafetyZoneResponseType::BadRequest(
                error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(AddVirtualControllerSafetyZoneResponseType::UndefinedResponse(response)),
    }
}
