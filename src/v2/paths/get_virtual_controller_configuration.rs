use ::reqwest;

use crate::v2::objects::virtual_robot_configuration::VirtualRobotConfiguration;

use crate::v2::objects::error::Error;

pub enum GetVirtualControllerConfigurationResponseType {
    Ok(VirtualRobotConfiguration),

    BadRequest(Error),

    NotFound(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct GetVirtualControllerConfigurationPathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct GetVirtualControllerConfigurationQueryParameters {}

pub async fn get_virtual_controller_configuration(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetVirtualControllerConfigurationPathParameters,
) -> Result<GetVirtualControllerConfigurationResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/controllers/{}/virtual-robot-configuration",
            server, path_parameters.cell, path_parameters.controller
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<VirtualRobotConfiguration>().await {
            Ok(virtual_robot_configuration) => Ok(
                GetVirtualControllerConfigurationResponseType::Ok(virtual_robot_configuration),
            ),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(GetVirtualControllerConfigurationResponseType::BadRequest(
                error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetVirtualControllerConfigurationResponseType::NotFound(
                error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetVirtualControllerConfigurationResponseType::UndefinedResponse(response)),
    }
}
