use ::reqwest;

use crate::v2::objects::error::Error;

pub enum DeleteProfinetIoResponseType {
    NotFound(Error),

    UndefinedResponse(reqwest::Response),

    BadRequest(Error),
}

pub struct DeleteProfinetIoPathParameters {
    pub cell: String,

    pub io: String,
}

pub struct DeleteProfinetIoQueryParameters {}

pub async fn delete_profinet_io(
    client: &reqwest::Client,

    server: &str,

    path_parameters: DeleteProfinetIoPathParameters,
) -> Result<DeleteProfinetIoResponseType, reqwest::Error> {
    let response = match client
        .delete(format!(
            "{}/cells/{}/bus-ios/profinet/ios/{}",
            server, path_parameters.cell, path_parameters.io
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        400 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteProfinetIoResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteProfinetIoResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(DeleteProfinetIoResponseType::UndefinedResponse(response)),
    }
}
