use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::list_coordinate_systems_response::ListCoordinateSystemsResponse;

pub enum ListVirtualControllerCoordinateSystemsResponseType {
    Ok(ListCoordinateSystemsResponse),

    UndefinedResponse(reqwest::Response),

    BadRequest(Error),
}

pub struct ListVirtualControllerCoordinateSystemsPathParameters {
    pub controller: String,

    pub cell: String,
}

pub struct ListVirtualControllerCoordinateSystemsQueryParameters {}

pub async fn list_virtual_controller_coordinate_systems(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ListVirtualControllerCoordinateSystemsPathParameters,
) -> Result<ListVirtualControllerCoordinateSystemsResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/virtual-controllers/{}/coordinate-systems",
            server, path_parameters.cell, path_parameters.controller
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        400 => match response.json::<Error>().await {
            Ok(error) => Ok(ListVirtualControllerCoordinateSystemsResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<ListCoordinateSystemsResponse>().await {
            Ok(list_coordinate_systems_response) => {
                Ok(ListVirtualControllerCoordinateSystemsResponseType::Ok(
                    list_coordinate_systems_response,
                ))
            }
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListVirtualControllerCoordinateSystemsResponseType::UndefinedResponse(response)),
    }
}
