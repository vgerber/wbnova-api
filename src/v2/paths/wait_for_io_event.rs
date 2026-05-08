use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::wait_for_io_event_request::WaitForIoEventRequest;

pub enum WaitForIoEventResponseType {
    NotFound(Error),

    BadRequest(Error),

    Ok(bool),

    UndefinedResponse(reqwest::Response),
}

pub struct WaitForIoEventPathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct WaitForIoEventQueryParameters {}

pub async fn wait_for_io_event(
    client: &reqwest::Client,

    server: &str,

    content: WaitForIoEventRequest,

    path_parameters: WaitForIoEventPathParameters,
) -> Result<WaitForIoEventResponseType, reqwest::Error> {
    let response = match client
        .post(format!(
            "{}/cells/{}/controllers/{}/ios/wait-for",
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
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(WaitForIoEventResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(WaitForIoEventResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<bool>().await {
            Ok(bool) => Ok(WaitForIoEventResponseType::Ok(bool)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(WaitForIoEventResponseType::UndefinedResponse(response)),
    }
}
