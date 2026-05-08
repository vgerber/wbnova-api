use ::reqwest;

use crate::v2::objects::coordinate_system::CoordinateSystem;

use crate::v2::objects::error::Error;

pub enum GetCoordinateSystemResponseType {
    Ok(CoordinateSystem),

    UndefinedResponse(reqwest::Response),

    NotFound(Error),

    BadRequest(Error),
}

pub struct GetCoordinateSystemPathParameters {
    pub coordinate_system: String,

    pub cell: String,

    pub controller: String,
}

pub struct GetCoordinateSystemQueryParameters {
    pub orientation_type: Option<String>,
}

pub async fn get_coordinate_system(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetCoordinateSystemPathParameters,

    query_parameters: GetCoordinateSystemQueryParameters,
) -> Result<GetCoordinateSystemResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.orientation_type {
        reqwest_query_parameters.push(("orientation_type", query_parameter.to_string()));
    }

    let response = match client
        .get(format!(
            "{}/cells/{}/controllers/{}/coordinate-systems/{}",
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
        200 => match response.json::<CoordinateSystem>().await {
            Ok(coordinate_system) => Ok(GetCoordinateSystemResponseType::Ok(coordinate_system)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetCoordinateSystemResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(GetCoordinateSystemResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetCoordinateSystemResponseType::UndefinedResponse(response)),
    }
}
