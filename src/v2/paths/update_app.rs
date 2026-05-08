use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::app::App;

pub enum UpdateAppResponseType {
    NotFound(Error),

    UndefinedResponse(reqwest::Response),

    BadRequest(Error),
}

pub struct UpdateAppPathParameters {
    pub cell: String,

    pub app: String,
}

pub struct UpdateAppQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn update_app(
    client: &reqwest::Client,

    server: &str,

    content: App,

    path_parameters: UpdateAppPathParameters,

    query_parameters: UpdateAppQueryParameters,
) -> Result<UpdateAppResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        reqwest_query_parameters.push(("completion_timeout", query_parameter.to_string()));
    }

    let response = match client
        .put(format!(
            "{}/cells/{}/apps/{}",
            server, path_parameters.cell, path_parameters.app
        ))
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
            Ok(error) => Ok(UpdateAppResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(UpdateAppResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(UpdateAppResponseType::UndefinedResponse(response)),
    }
}
