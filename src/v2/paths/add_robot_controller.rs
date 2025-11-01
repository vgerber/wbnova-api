use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::robot_controller::RobotController;

pub enum AddRobotControllerResponseType {
    NotFound(Error),

    Forbidden(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct AddRobotControllerPathParameters {
    pub cell: String,
}

pub struct AddRobotControllerQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn add_robot_controller(
    client: &reqwest::Client,

    server: &str,

    content: RobotController,

    path_parameters: AddRobotControllerPathParameters,

    query_parameters: AddRobotControllerQueryParameters,
) -> Result<AddRobotControllerResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        reqwest_query_parameters.push(("completion_timeout", query_parameter.to_string()));
    }

    let response = match client
        .post(format!(
            "{}/cells/{}/controllers",
            server, path_parameters.cell
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
        403 => match response.json::<Error>().await {
            Ok(error) => Ok(AddRobotControllerResponseType::Forbidden(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(AddRobotControllerResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(AddRobotControllerResponseType::UndefinedResponse(response)),
    }
}
