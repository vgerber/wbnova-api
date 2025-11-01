use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::profinet_io_data::ProfinetIoData;

pub enum AddProfinetIoResponseType {
    BadRequest(Error),

    NotFound(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct AddProfinetIoPathParameters {
    pub io: String,

    pub cell: String,
}

pub struct AddProfinetIoQueryParameters {}

pub async fn add_profinet_io(
    client: &reqwest::Client,

    server: &str,

    content: ProfinetIoData,

    path_parameters: AddProfinetIoPathParameters,
) -> Result<AddProfinetIoResponseType, reqwest::Error> {
    let response = match client
        .put(format!(
            "{}/cells/{}/bus-ios/profinet/ios/{}",
            server, path_parameters.cell, path_parameters.io
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
            Ok(error) => Ok(AddProfinetIoResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(AddProfinetIoResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(AddProfinetIoResponseType::UndefinedResponse(response)),
    }
}
