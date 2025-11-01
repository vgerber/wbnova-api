use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::list_coordinate_systems_response::ListCoordinateSystemsResponse;

pub enum ListCoordinateSystemsResponseType {
    BadRequest(Error),

    Ok(ListCoordinateSystemsResponse),

    UndefinedResponse(reqwest::Response),
}

pub struct ListCoordinateSystemsPathParameters {
    pub controller: String,

    pub cell: String,
}

pub struct ListCoordinateSystemsQueryParameters {
    pub orientation_type: Option<String>,
}

pub async fn list_coordinate_systems(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ListCoordinateSystemsPathParameters,

    query_parameters: ListCoordinateSystemsQueryParameters,
) -> Result<ListCoordinateSystemsResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.orientation_type {
        reqwest_query_parameters.push(("orientation_type", query_parameter.to_string()));
    }

    let response = match client
        .get(format!(
            "{}/cells/{}/controllers/{}/coordinate-systems",
            server, path_parameters.cell, path_parameters.controller
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
            Ok(error) => Ok(ListCoordinateSystemsResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<ListCoordinateSystemsResponse>().await {
            Ok(list_coordinate_systems_response) => Ok(ListCoordinateSystemsResponseType::Ok(
                list_coordinate_systems_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListCoordinateSystemsResponseType::UndefinedResponse(
            response,
        )),
    }
}
