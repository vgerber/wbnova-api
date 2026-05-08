use ::reqwest;

use crate::v2::objects::error::Error;

pub enum GetProfinetGsdmlResponseType {
    PreconditionFailed(Error),

    NotFound(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct GetProfinetGsdmlPathParameters {
    pub cell: String,
}

pub struct GetProfinetGsdmlQueryParameters {}

pub async fn get_profinet_gsdml(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetProfinetGsdmlPathParameters,
) -> Result<GetProfinetGsdmlResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/bus-ios/profinet/gsdml",
            server, path_parameters.cell
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        412 => match response.json::<Error>().await {
            Ok(error) => Ok(GetProfinetGsdmlResponseType::PreconditionFailed(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetProfinetGsdmlResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetProfinetGsdmlResponseType::UndefinedResponse(response)),
    }
}
