use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::update_cell_version_request::UpdateCellVersionRequest;

pub enum UpdateCellVersionResponseType {
    NotFound(Error),

    UndefinedResponse(reqwest::Response),

    BadRequest(Error),
}

pub struct UpdateCellVersionPathParameters {
    pub cell: String,
}

pub struct UpdateCellVersionQueryParameters {}

pub async fn update_cell_version(
    client: &reqwest::Client,

    server: &str,

    content: UpdateCellVersionRequest,

    path_parameters: UpdateCellVersionPathParameters,
) -> Result<UpdateCellVersionResponseType, reqwest::Error> {
    let response = match client
        .put(format!("{}/cells/{}/update", server, path_parameters.cell))
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(UpdateCellVersionResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(UpdateCellVersionResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(UpdateCellVersionResponseType::UndefinedResponse(response)),
    }
}
