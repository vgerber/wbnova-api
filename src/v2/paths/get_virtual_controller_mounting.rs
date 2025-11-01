use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::coordinate_system::CoordinateSystem;

pub enum GetVirtualControllerMountingResponseType {
    NotFound(Error),

    Ok(CoordinateSystem),

    BadRequest(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct GetVirtualControllerMountingPathParameters {
    pub motion_group: String,

    pub controller: String,

    pub cell: String,
}

pub struct GetVirtualControllerMountingQueryParameters {}

pub async fn get_virtual_controller_mounting(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetVirtualControllerMountingPathParameters,
) -> Result<GetVirtualControllerMountingResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/virtual-controllers/{}/motion-groups/{}/mounting",
            server, path_parameters.cell, path_parameters.controller, path_parameters.motion_group
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetVirtualControllerMountingResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<CoordinateSystem>().await {
            Ok(coordinate_system) => Ok(GetVirtualControllerMountingResponseType::Ok(
                coordinate_system,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(GetVirtualControllerMountingResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetVirtualControllerMountingResponseType::UndefinedResponse(
            response,
        )),
    }
}
