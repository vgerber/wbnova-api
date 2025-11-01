use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::behavior::Behavior;

pub enum GetVirtualControllerBehaviorResponseType {
    UndefinedResponse(reqwest::Response),

    Ok(Behavior),

    NotFound(Error),

    BadRequest(Error),
}

pub struct GetVirtualControllerBehaviorPathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct GetVirtualControllerBehaviorQueryParameters {}

pub async fn get_virtual_controller_behavior(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetVirtualControllerBehaviorPathParameters,
) -> Result<GetVirtualControllerBehaviorResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/virtual-controllers/{}/behavior",
            server, path_parameters.cell, path_parameters.controller
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetVirtualControllerBehaviorResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(GetVirtualControllerBehaviorResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<Behavior>().await {
            Ok(behavior) => Ok(GetVirtualControllerBehaviorResponseType::Ok(behavior)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetVirtualControllerBehaviorResponseType::UndefinedResponse(
            response,
        )),
    }
}
