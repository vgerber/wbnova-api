use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::robot_controller::RobotController;

pub enum UpdateRobotControllerResponseType {
    UndefinedResponse(reqwest::Response),

    NotFound(Error),
}

pub struct UpdateRobotControllerPathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct UpdateRobotControllerQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn update_robot_controller(
    client: &reqwest::Client,

    server: &str,

    content: RobotController,

    path_parameters: UpdateRobotControllerPathParameters,

    query_parameters: UpdateRobotControllerQueryParameters,
) -> Result<UpdateRobotControllerResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        reqwest_query_parameters.push(("completion_timeout", query_parameter.to_string()));
    }

    let response = match client
        .put(format!(
            "{}/cells/{}/controllers/{}",
            server, path_parameters.cell, path_parameters.controller
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
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(UpdateRobotControllerResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(UpdateRobotControllerResponseType::UndefinedResponse(
            response,
        )),
    }
}
