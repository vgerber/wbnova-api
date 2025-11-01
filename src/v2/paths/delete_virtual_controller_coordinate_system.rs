use ::reqwest;

use crate::v2::objects::error::Error;

pub enum DeleteVirtualControllerCoordinateSystemResponseType {
    UndefinedResponse(reqwest::Response),

    NotFound(Error),

    BadRequest(Error),
}

pub struct DeleteVirtualControllerCoordinateSystemPathParameters {
    pub controller: String,

    pub cell: String,

    pub coordinate_system: String,
}

pub struct DeleteVirtualControllerCoordinateSystemQueryParameters {
    pub delete_dependent: Option<bool>,
}

pub async fn delete_virtual_controller_coordinate_system(
    client: &reqwest::Client,

    server: &str,

    path_parameters: DeleteVirtualControllerCoordinateSystemPathParameters,

    query_parameters: DeleteVirtualControllerCoordinateSystemQueryParameters,
) -> Result<DeleteVirtualControllerCoordinateSystemResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.delete_dependent {
        reqwest_query_parameters.push(("delete_dependent", query_parameter.to_string()));
    }

    let response = match client
        .delete(format!(
            "{}/cells/{}/virtual-controllers/{}/coordinate-systems/{}",
            server,
            path_parameters.cell,
            path_parameters.controller,
            path_parameters.coordinate_system
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
            Ok(error) => Ok(DeleteVirtualControllerCoordinateSystemResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteVirtualControllerCoordinateSystemResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(DeleteVirtualControllerCoordinateSystemResponseType::UndefinedResponse(response)),
    }
}
