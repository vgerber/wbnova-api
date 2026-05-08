use ::reqwest;

use crate::v2::objects::error::Error;

pub enum DeleteAppResponseType {
    UndefinedResponse(reqwest::Response),

    NotFound(Error),
}

pub struct DeleteAppPathParameters {
    pub cell: String,

    pub app: String,
}

pub struct DeleteAppQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn delete_app(
    client: &reqwest::Client,

    server: &str,

    path_parameters: DeleteAppPathParameters,

    query_parameters: DeleteAppQueryParameters,
) -> Result<DeleteAppResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        reqwest_query_parameters.push(("completion_timeout", query_parameter.to_string()));
    }

    let response = match client
        .delete(format!(
            "{}/cells/{}/apps/{}",
            server, path_parameters.cell, path_parameters.app
        ))
        .query(&reqwest_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteAppResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(DeleteAppResponseType::UndefinedResponse(response)),
    }
}
