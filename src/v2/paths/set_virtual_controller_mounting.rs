use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::coordinate_system::CoordinateSystem;

pub enum SetVirtualControllerMountingResponseType {
    BadRequest(Error),

    UndefinedResponse(reqwest::Response),

    Ok(CoordinateSystem),

    NotFound(Error),
}

pub struct SetVirtualControllerMountingPathParameters {
    pub cell: String,

    pub motion_group: String,

    pub controller: String,
}

pub struct SetVirtualControllerMountingQueryParameters {}

pub async fn set_virtual_controller_mounting(
    client: &reqwest::Client,

    server: &str,

    content: CoordinateSystem,

    path_parameters: SetVirtualControllerMountingPathParameters,
) -> Result<SetVirtualControllerMountingResponseType, reqwest::Error> {
    let response = match client
        .put(format!(
            "{}/cells/{}/virtual-controllers/{}/motion-groups/{}/mounting",
            server, path_parameters.cell, path_parameters.controller, path_parameters.motion_group
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
            Ok(error) => Ok(SetVirtualControllerMountingResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(SetVirtualControllerMountingResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<CoordinateSystem>().await {
            Ok(coordinate_system) => Ok(SetVirtualControllerMountingResponseType::Ok(
                coordinate_system,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(SetVirtualControllerMountingResponseType::UndefinedResponse(
            response,
        )),
    }
}
