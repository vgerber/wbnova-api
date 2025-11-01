use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::execute_trajectory_response::ExecuteTrajectoryResponse;

use crate::v2::objects::execute_trajectory_request::ExecuteTrajectoryRequest;

pub enum ExecuteTrajectoryResponseType {
    BadRequest(Error),

    Ok(ExecuteTrajectoryResponse),

    UndefinedResponse(reqwest::Response),

    NotFound(Error),
}

pub struct ExecuteTrajectoryPathParameters {
    pub controller: String,

    pub cell: String,
}

pub struct ExecuteTrajectoryQueryParameters {}

pub async fn execute_trajectory(
    client: &reqwest::Client,

    server: &str,

    content: ExecuteTrajectoryRequest,

    path_parameters: ExecuteTrajectoryPathParameters,
) -> Result<ExecuteTrajectoryResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/controllers/{}/execution/trajectory",
            server, path_parameters.cell, path_parameters.controller
        ))
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        400 => match response.json::<Error>().await {
            Ok(error) => Ok(ExecuteTrajectoryResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(ExecuteTrajectoryResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<ExecuteTrajectoryResponse>().await {
            Ok(execute_trajectory_response) => Ok(ExecuteTrajectoryResponseType::Ok(
                execute_trajectory_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ExecuteTrajectoryResponseType::UndefinedResponse(response)),
    }
}
