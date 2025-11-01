use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::get_trajectory_response::GetTrajectoryResponse;

pub enum GetTrajectoryResponseType {
    NotFound(Error),

    BadRequest(Error),

    Ok(GetTrajectoryResponse),

    UndefinedResponse(reqwest::Response),
}

pub struct GetTrajectoryPathParameters {
    pub cell: String,

    pub controller: String,

    pub trajectory: String,
}

pub struct GetTrajectoryQueryParameters {}

pub async fn get_trajectory(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetTrajectoryPathParameters,
) -> Result<GetTrajectoryResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/controllers/{}/trajectories/{}",
            server, path_parameters.cell, path_parameters.controller, path_parameters.trajectory
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        400 => match response.json::<Error>().await {
            Ok(error) => Ok(GetTrajectoryResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<GetTrajectoryResponse>().await {
            Ok(get_trajectory_response) => {
                Ok(GetTrajectoryResponseType::Ok(get_trajectory_response))
            }
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetTrajectoryResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetTrajectoryResponseType::UndefinedResponse(response)),
    }
}
