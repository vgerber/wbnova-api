use ::reqwest;

use crate::v2::objects::error::Error;

pub enum DeleteVirtualControllerSafetyZoneResponseType {
    NotFound(Error),

    UndefinedResponse(reqwest::Response),

    BadRequest(Error),
}

pub struct DeleteVirtualControllerSafetyZonePathParameters {
    pub controller: String,

    pub id: String,

    pub cell: String,
}

pub struct DeleteVirtualControllerSafetyZoneQueryParameters {}

pub async fn delete_virtual_controller_safety_zone(
    client: &reqwest::Client,

    server: &str,

    path_parameters: DeleteVirtualControllerSafetyZonePathParameters,
) -> Result<DeleteVirtualControllerSafetyZoneResponseType, reqwest::Error> {
    let response = match client
        .delete(format!(
            "{}/cells/{}/virtual-controllers/{}/safety-zones/{}",
            server, path_parameters.cell, path_parameters.controller, path_parameters.id
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteVirtualControllerSafetyZoneResponseType::NotFound(
                error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteVirtualControllerSafetyZoneResponseType::BadRequest(
                error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(DeleteVirtualControllerSafetyZoneResponseType::UndefinedResponse(response)),
    }
}
