use ::reqwest;

use crate::v2::objects::error::Error;

pub enum GetProfinetIOsFromFileResponseType {
    NotFound(Error),

    UndefinedResponse(reqwest::Response),

    BadRequest(Error),

    Ok(String),
}

pub struct GetProfinetIOsFromFilePathParameters {
    pub cell: String,
}

pub struct GetProfinetIOsFromFileQueryParameters {
    pub input_offset: Option<i32>,

    pub output_offset: Option<i32>,
}

pub async fn get_profinet_i_os_from_file(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetProfinetIOsFromFilePathParameters,

    query_parameters: GetProfinetIOsFromFileQueryParameters,
) -> Result<GetProfinetIOsFromFileResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.input_offset {
        reqwest_query_parameters.push(("input_offset", query_parameter.to_string()));
    }

    if let Some(ref query_parameter) = query_parameters.output_offset {
        reqwest_query_parameters.push(("output_offset", query_parameter.to_string()));
    }

    let response = match client
        .get(format!(
            "{}/cells/{}/bus-ios/profinet/iofile",
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
        400 => match response.json::<Error>().await {
            Ok(error) => Ok(GetProfinetIOsFromFileResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetProfinetIOsFromFileResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<String>().await {
            Ok(string) => Ok(GetProfinetIOsFromFileResponseType::Ok(string)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetProfinetIOsFromFileResponseType::UndefinedResponse(
            response,
        )),
    }
}
