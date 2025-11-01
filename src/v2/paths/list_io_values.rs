use ::reqwest;

use crate::v2::objects::list_io_values_response::ListIoValuesResponse;

use crate::v2::objects::error::Error;

pub enum ListIoValuesResponseType {
    NotFound(Error),

    Ok(ListIoValuesResponse),

    UndefinedResponse(reqwest::Response),

    BadRequest(Error),
}

pub struct ListIoValuesPathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct ListIoValuesQueryParameters {
    pub ios: Option<Vec<String>>,
}

pub async fn list_io_values(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ListIoValuesPathParameters,

    query_parameters: ListIoValuesQueryParameters,
) -> Result<ListIoValuesResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.ios {
        query_parameter.iter().for_each(|query_parameter_item| {
            reqwest_query_parameters.push(("ios", query_parameter_item.to_string()))
        });
    }

    let response = match client
        .get(format!(
            "{}/cells/{}/controllers/{}/ios/values",
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
        200 => match response.json::<ListIoValuesResponse>().await {
            Ok(list_io_values_response) => {
                Ok(ListIoValuesResponseType::Ok(list_io_values_response))
            }
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(ListIoValuesResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(ListIoValuesResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListIoValuesResponseType::UndefinedResponse(response)),
    }
}
