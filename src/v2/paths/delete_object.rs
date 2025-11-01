use ::reqwest;

use crate::v2::objects::error::Error;

pub enum DeleteObjectResponseType {
    NotFound(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct DeleteObjectPathParameters {
    pub cell: String,

    pub key: String,
}

pub struct DeleteObjectQueryParameters {}

pub async fn delete_object(
    client: &reqwest::Client,

    server: &str,

    path_parameters: DeleteObjectPathParameters,
) -> Result<DeleteObjectResponseType, reqwest::Error> {
    let response = match client
        .delete(format!(
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
            Ok(error) => Ok(DeleteObjectResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(DeleteObjectResponseType::UndefinedResponse(response)),
    }
}
