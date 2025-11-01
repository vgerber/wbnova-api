use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::list_io_values_response::ListIoValuesResponse;

pub enum GetBusIoValuesResponseType {
    BadRequest(Error),

    Ok(ListIoValuesResponse),

    NotFound(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct GetBusIoValuesPathParameters {
    pub cell: String,
}

pub struct GetBusIoValuesQueryParameters {
    pub ios: Option<Vec<String>>,
}

pub async fn get_bus_io_values(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetBusIoValuesPathParameters,

    query_parameters: GetBusIoValuesQueryParameters,
) -> Result<GetBusIoValuesResponseType, reqwest::Error> {
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
            "{}/cells/{}/bus-ios/ios/values",
            server, path_parameters.cell
        ))
        .query(&reqwest_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetBusIoValuesResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<ListIoValuesResponse>().await {
            Ok(list_io_values_response) => {
                Ok(GetBusIoValuesResponseType::Ok(list_io_values_response))
            }
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(GetBusIoValuesResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetBusIoValuesResponseType::UndefinedResponse(response)),
    }
}
