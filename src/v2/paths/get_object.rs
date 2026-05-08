use ::reqwest;

use crate::v2::objects::error::Error;

pub enum GetObjectResponseType {
    NotFound(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct GetObjectPathParameters {
    pub key: String,

    pub cell: String,
}

pub struct GetObjectQueryParameters {}

pub async fn get_object(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetObjectPathParameters,
) -> Result<GetObjectResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/store/objects/{}",
            server, path_parameters.cell, path_parameters.key
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetObjectResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetObjectResponseType::UndefinedResponse(response)),
    }
}
