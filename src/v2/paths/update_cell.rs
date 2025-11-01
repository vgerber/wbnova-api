use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::cell::Cell;

pub enum UpdateCellResponseType {
    Forbidden(Error),

    BadRequest(Error),

    NotFound(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct UpdateCellPathParameters {
    pub cell: String,
}

pub struct UpdateCellQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn update_cell(
    client: &reqwest::Client,

    server: &str,

    content: Cell,

    path_parameters: UpdateCellPathParameters,

    query_parameters: UpdateCellQueryParameters,
) -> Result<UpdateCellResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        reqwest_query_parameters.push(("completion_timeout", query_parameter.to_string()));
    }

    let response = match client
        .put(format!("{}/cells/{}", server, path_parameters.cell))
        .query(&reqwest_query_parameters)
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        400 => match response.json::<Error>().await {
            Ok(error) => Ok(UpdateCellResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        403 => match response.json::<Error>().await {
            Ok(error) => Ok(UpdateCellResponseType::Forbidden(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(UpdateCellResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(UpdateCellResponseType::UndefinedResponse(response)),
    }
}
