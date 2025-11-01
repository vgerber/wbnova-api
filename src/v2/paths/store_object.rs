use ::reqwest;

use crate::v2::objects::error::Error;

pub enum StoreObjectResponseType {
    BadRequest(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct StoreObjectPathParameters {
    pub key: String,

    pub cell: String,
}

pub struct StoreObjectQueryParameters {}

pub async fn store_object(
    client: &reqwest::Client,

    server: &str,

    path_parameters: StoreObjectPathParameters,
) -> Result<StoreObjectResponseType, reqwest::Error> {
    let response = match client
        .put(format!(
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
        400 => match response.json::<Error>().await {
            Ok(error) => Ok(StoreObjectResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(StoreObjectResponseType::UndefinedResponse(response)),
    }
}
