use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::bus_io_type::BusIoType;

pub enum AddBusIoServiceResponseType {
    NotFound(Error),

    UndefinedResponse(reqwest::Response),

    Forbidden(Error),
}

pub struct AddBusIoServicePathParameters {
    pub cell: String,
}

pub struct AddBusIoServiceQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn add_bus_io_service(
    client: &reqwest::Client,

    server: &str,

    content: BusIoType,

    path_parameters: AddBusIoServicePathParameters,

    query_parameters: AddBusIoServiceQueryParameters,
) -> Result<AddBusIoServiceResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        reqwest_query_parameters.push(("completion_timeout", query_parameter.to_string()));
    }

    let response = match client
        .post(format!("{}/cells/{}/bus-ios", server, path_parameters.cell))
        .query(&reqwest_query_parameters)
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        403 => match response.json::<Error>().await {
            Ok(error) => Ok(AddBusIoServiceResponseType::Forbidden(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(AddBusIoServiceResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(AddBusIoServiceResponseType::UndefinedResponse(response)),
    }
}
