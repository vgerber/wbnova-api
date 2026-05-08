use ::reqwest;

use crate::v2::objects::controller_description::ControllerDescription;

use crate::v2::objects::error::Error;

pub enum GetControllerDescriptionResponseType {
    UndefinedResponse(reqwest::Response),

    Ok(ControllerDescription),

    NotFound(Error),

    BadRequest(Error),
}

pub struct GetControllerDescriptionPathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct GetControllerDescriptionQueryParameters {}

pub async fn get_controller_description(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetControllerDescriptionPathParameters,
) -> Result<GetControllerDescriptionResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/controllers/{}/description",
            server, path_parameters.cell, path_parameters.controller
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<ControllerDescription>().await {
            Ok(controller_description) => Ok(GetControllerDescriptionResponseType::Ok(
                controller_description,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetControllerDescriptionResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(GetControllerDescriptionResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetControllerDescriptionResponseType::UndefinedResponse(
            response,
        )),
    }
}
