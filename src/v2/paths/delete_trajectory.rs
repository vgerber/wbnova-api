use ::reqwest;

use crate::v2::objects::error::Error;

pub enum DeleteTrajectoryResponseType {
    BadRequest(Error),

    NotFound(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct DeleteTrajectoryPathParameters {
    pub trajectory: String,

    pub controller: String,

    pub cell: String,
}

pub struct DeleteTrajectoryQueryParameters {}

pub async fn delete_trajectory(
    client: &reqwest::Client,

    server: &str,

    path_parameters: DeleteTrajectoryPathParameters,
) -> Result<DeleteTrajectoryResponseType, reqwest::Error> {
    let response = match client
        .delete(format!(
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
            Ok(error) => Ok(DeleteTrajectoryResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteTrajectoryResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(DeleteTrajectoryResponseType::UndefinedResponse(response)),
    }
}
