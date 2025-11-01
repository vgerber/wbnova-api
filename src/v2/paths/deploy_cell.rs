use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::cell::Cell;

pub enum DeployCellResponseType {
    Forbidden(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct DeployCellPathParameters {}

pub struct DeployCellQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn deploy_cell(
    client: &reqwest::Client,

    server: &str,

    content: Cell,

    query_parameters: DeployCellQueryParameters,
) -> Result<DeployCellResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        reqwest_query_parameters.push(("completion_timeout", query_parameter.to_string()));
    }

    let response = match client
        .post(format!("{}/cells", server,))
        .query(&reqwest_query_parameters)
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        403 => match response.json::<Error>().await {
            Ok(error) => Ok(DeployCellResponseType::Forbidden(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(DeployCellResponseType::UndefinedResponse(response)),
    }
}
