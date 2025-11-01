use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::add_trajectory_response::AddTrajectoryResponse;

use crate::v2::objects::add_trajectory_request::AddTrajectoryRequest;

pub enum AddTrajectoryResponseType {
    BadRequest(Error),

    NotFound(Error),

    Ok(AddTrajectoryResponse),

    UndefinedResponse(reqwest::Response),
}

pub struct AddTrajectoryPathParameters {
    pub controller: String,

    pub cell: String,
}

pub struct AddTrajectoryQueryParameters {}

pub async fn add_trajectory(
    client: &reqwest::Client,

    server: &str,

    content: AddTrajectoryRequest,

    path_parameters: AddTrajectoryPathParameters,
) -> Result<AddTrajectoryResponseType, reqwest::Error> {
    let response = match client
        .post(format!(
            "{}/cells/{}/controllers/{}/trajectories",
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
            Ok(error) => Ok(AddTrajectoryResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(AddTrajectoryResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<AddTrajectoryResponse>().await {
            Ok(add_trajectory_response) => {
                Ok(AddTrajectoryResponseType::Ok(add_trajectory_response))
            }
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(AddTrajectoryResponseType::UndefinedResponse(response)),
    }
}
