use ::reqwest;

use crate::v2::objects::app::App;

pub enum AddAppResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct AddAppPathParameters {
    pub cell: String,
}

pub struct AddAppQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn add_app(
    client: &reqwest::Client,

    server: &str,

    content: App,

    path_parameters: AddAppPathParameters,

    query_parameters: AddAppQueryParameters,
) -> Result<AddAppResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        reqwest_query_parameters.push(("completion_timeout", query_parameter.to_string()));
    }

    let response = match client
        .post(format!("{}/cells/{}/apps", server, path_parameters.cell))
        .query(&reqwest_query_parameters)
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        _ => Ok(AddAppResponseType::UndefinedResponse(response)),
    }
}
