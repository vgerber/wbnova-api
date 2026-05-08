use ::reqwest;

use crate::v2::objects::error::Error;

pub enum DeleteRobotControllerResponseType {
    NotFound(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct DeleteRobotControllerPathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct DeleteRobotControllerQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn delete_robot_controller(
    client: &reqwest::Client,

    server: &str,

    path_parameters: DeleteRobotControllerPathParameters,

    query_parameters: DeleteRobotControllerQueryParameters,
) -> Result<DeleteRobotControllerResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        reqwest_query_parameters.push(("completion_timeout", query_parameter.to_string()));
    }

    let response = match client
        .delete(format!(
            "{}/cells/{}/controllers/{}",
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
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteRobotControllerResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(DeleteRobotControllerResponseType::UndefinedResponse(
            response,
        )),
    }
}
