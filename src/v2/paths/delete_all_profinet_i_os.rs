use ::reqwest;

use crate::v2::objects::error::Error;

pub enum DeleteAllProfinetIOsResponseType {
    NotFound(Error),

    PreconditionFailed(Error),

    BadRequest(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct DeleteAllProfinetIOsPathParameters {
    pub cell: String,
}

pub struct DeleteAllProfinetIOsQueryParameters {}

pub async fn delete_all_profinet_i_os(
    client: &reqwest::Client,

    server: &str,

    path_parameters: DeleteAllProfinetIOsPathParameters,
) -> Result<DeleteAllProfinetIOsResponseType, reqwest::Error> {
    let response = match client
        .delete(format!(
            "{}/cells/{}/bus-ios/profinet/ios",
            server, path_parameters.cell
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteAllProfinetIOsResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        412 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteAllProfinetIOsResponseType::PreconditionFailed(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteAllProfinetIOsResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(DeleteAllProfinetIOsResponseType::UndefinedResponse(
            response,
        )),
    }
}
