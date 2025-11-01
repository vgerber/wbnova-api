use ::reqwest;

use crate::v2::objects::error::Error;

pub enum ClearBusIoServiceResponseType {
    UndefinedResponse(reqwest::Response),

    NotFound(Error),
}

pub struct ClearBusIoServicePathParameters {
    pub cell: String,
}

pub struct ClearBusIoServiceQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn clear_bus_io_service(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ClearBusIoServicePathParameters,

    query_parameters: ClearBusIoServiceQueryParameters,
) -> Result<ClearBusIoServiceResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        reqwest_query_parameters.push(("completion_timeout", query_parameter.to_string()));
    }

    let response = match client
        .delete(format!("{}/cells/{}/bus-ios", server, path_parameters.cell))
        .query(&reqwest_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(ClearBusIoServiceResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ClearBusIoServiceResponseType::UndefinedResponse(response)),
    }
}
