use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::robot_controller::RobotController;

pub enum GetRobotControllerResponseType {
    UndefinedResponse(reqwest::Response),

    NotFound(Error),

    Ok(RobotController),
}

pub struct GetRobotControllerPathParameters {
    pub controller: String,

    pub cell: String,
}

pub struct GetRobotControllerQueryParameters {}

pub async fn get_robot_controller(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetRobotControllerPathParameters,
) -> Result<GetRobotControllerResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/controllers/{}",
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
            Ok(error) => Ok(GetRobotControllerResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<RobotController>().await {
            Ok(robot_controller) => Ok(GetRobotControllerResponseType::Ok(robot_controller)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetRobotControllerResponseType::UndefinedResponse(response)),
    }
}
