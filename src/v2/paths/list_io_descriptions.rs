use ::reqwest;

use crate::v2::objects::list_io_descriptions_response::ListIoDescriptionsResponse;

use crate::v2::objects::error::Error;

pub enum ListIoDescriptionsResponseType {
    UndefinedResponse(reqwest::Response),

    NotFound(Error),

    BadRequest(Error),

    Ok(ListIoDescriptionsResponse),
}

pub struct ListIoDescriptionsPathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct ListIoDescriptionsQueryParameters {
    pub direction: Option<String>,

    pub ios: Option<Vec<String>>,

    pub group: Option<String>,

    pub value_type: Option<String>,
}

pub async fn list_io_descriptions(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ListIoDescriptionsPathParameters,

    query_parameters: ListIoDescriptionsQueryParameters,
) -> Result<ListIoDescriptionsResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.direction {
        reqwest_query_parameters.push(("direction", query_parameter.to_string()));
    }

    if let Some(ref query_parameter) = query_parameters.ios {
        query_parameter.iter().for_each(|query_parameter_item| {
            reqwest_query_parameters.push(("ios", query_parameter_item.to_string()))
        });
    }

    if let Some(ref query_parameter) = query_parameters.group {
        reqwest_query_parameters.push(("group", query_parameter.to_string()));
    }

    if let Some(ref query_parameter) = query_parameters.value_type {
        reqwest_query_parameters.push(("value_type", query_parameter.to_string()));
    }

    let response = match client
        .get(format!(
            "{}/cells/{}/controllers/{}/ios/description",
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
        200 => match response.json::<ListIoDescriptionsResponse>().await {
            Ok(list_io_descriptions_response) => Ok(ListIoDescriptionsResponseType::Ok(
                list_io_descriptions_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(ListIoDescriptionsResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(ListIoDescriptionsResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListIoDescriptionsResponseType::UndefinedResponse(response)),
    }
}
