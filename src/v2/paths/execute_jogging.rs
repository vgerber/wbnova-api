use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::execute_jogging_response::ExecuteJoggingResponse;

use crate::v2::objects::execute_jogging_request::ExecuteJoggingRequest;

pub enum ExecuteJoggingResponseType {
    NotFound(Error),

    Ok(ExecuteJoggingResponse),

    BadRequest(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct ExecuteJoggingPathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct ExecuteJoggingQueryParameters {}

pub async fn execute_jogging(
    client: &reqwest::Client,

    server: &str,

    content: ExecuteJoggingRequest,

    path_parameters: ExecuteJoggingPathParameters,
) -> Result<ExecuteJoggingResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/controllers/{}/execution/jogging",
            server, path_parameters.cell, path_parameters.controller
        ))
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        400 => match response.json::<Error>().await {
            Ok(error) => Ok(ExecuteJoggingResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(ExecuteJoggingResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<ExecuteJoggingResponse>().await {
            Ok(execute_jogging_response) => {
                Ok(ExecuteJoggingResponseType::Ok(execute_jogging_response))
            }
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ExecuteJoggingResponseType::UndefinedResponse(response)),
    }
}
