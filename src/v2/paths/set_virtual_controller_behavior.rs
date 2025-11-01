use ::reqwest;

use crate::v2::objects::error::Error;

pub enum SetVirtualControllerBehaviorResponseType {
    BadRequest(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct SetVirtualControllerBehaviorPathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct SetVirtualControllerBehaviorQueryParameters {
    pub behavior: Option<String>,
}

pub async fn set_virtual_controller_behavior(
    client: &reqwest::Client,

    server: &str,

    path_parameters: SetVirtualControllerBehaviorPathParameters,

    query_parameters: SetVirtualControllerBehaviorQueryParameters,
) -> Result<SetVirtualControllerBehaviorResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.behavior {
        reqwest_query_parameters.push(("behavior", query_parameter.to_string()));
    }

    let response = match client
        .put(format!(
            "{}/cells/{}/virtual-controllers/{}/behavior",
            server, path_parameters.cell, path_parameters.controller
        ))
        .query(&reqwest_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        400 => match response.json::<Error>().await {
            Ok(error) => Ok(SetVirtualControllerBehaviorResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(SetVirtualControllerBehaviorResponseType::UndefinedResponse(
            response,
        )),
    }
}
