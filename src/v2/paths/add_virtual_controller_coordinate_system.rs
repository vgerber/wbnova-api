use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::coordinate_system_data::CoordinateSystemData;

pub enum AddVirtualControllerCoordinateSystemResponseType {
    NotFound(Error),

    UndefinedResponse(reqwest::Response),

    BadRequest(Error),
}

pub struct AddVirtualControllerCoordinateSystemPathParameters {
    pub controller: String,

    pub coordinate_system: String,

    pub cell: String,
}

pub struct AddVirtualControllerCoordinateSystemQueryParameters {}

pub async fn add_virtual_controller_coordinate_system(
    client: &reqwest::Client,

    server: &str,

    content: CoordinateSystemData,

    path_parameters: AddVirtualControllerCoordinateSystemPathParameters,
) -> Result<AddVirtualControllerCoordinateSystemResponseType, reqwest::Error> {
    let response = match client
        .put(format!(
            "{}/cells/{}/virtual-controllers/{}/coordinate-systems/{}",
            server,
            path_parameters.cell,
            path_parameters.controller,
            path_parameters.coordinate_system
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
            Ok(error) => Ok(AddVirtualControllerCoordinateSystemResponseType::NotFound(
                error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(AddVirtualControllerCoordinateSystemResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(AddVirtualControllerCoordinateSystemResponseType::UndefinedResponse(response)),
    }
}
