use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::robot_controller_state::RobotControllerState;

pub enum GetCurrentRobotControllerStateResponseType {
    BadRequest(Error),

    NotFound(Error),

    UndefinedResponse(reqwest::Response),

    Ok(RobotControllerState),
}

pub struct GetCurrentRobotControllerStatePathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct GetCurrentRobotControllerStateQueryParameters {}

pub async fn get_current_robot_controller_state(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetCurrentRobotControllerStatePathParameters,
) -> Result<GetCurrentRobotControllerStateResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/controllers/{}/state",
            server, path_parameters.cell, path_parameters.controller
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        400 => match response.json::<Error>().await {
            Ok(error) => Ok(GetCurrentRobotControllerStateResponseType::BadRequest(
                error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<RobotControllerState>().await {
            Ok(robot_controller_state) => Ok(GetCurrentRobotControllerStateResponseType::Ok(
                robot_controller_state,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetCurrentRobotControllerStateResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetCurrentRobotControllerStateResponseType::UndefinedResponse(response)),
    }
}
