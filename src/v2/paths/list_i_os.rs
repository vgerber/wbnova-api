use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::list_io_values_response::ListIoValuesResponse;

pub enum ListIOsResponseType {
    NotFound(Error),

    Ok(ListIoValuesResponse),

    BadRequest(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct ListIOsPathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct ListIOsQueryParameters {
    pub ios: Vec<String>,
}

pub async fn list_i_os(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ListIOsPathParameters,

    query_parameters: ListIOsQueryParameters,
) -> Result<ListIOsResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Required Array Query Parameters

    query_parameters
        .ios
        .iter()
        .for_each(|query_parameter_item| {
            reqwest_query_parameters.push(("ios", query_parameter_item.to_string()))
        });

    let response = match client
        .get(format!(
            "{}/cells/{}/virtual-controllers/{}/ios/values",
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
            Ok(error) => Ok(ListIOsResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<ListIoValuesResponse>().await {
            Ok(list_io_values_response) => Ok(ListIOsResponseType::Ok(list_io_values_response)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(ListIOsResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListIOsResponseType::UndefinedResponse(response)),
    }
}
