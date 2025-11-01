use ::reqwest;

use crate::v2::objects::error::Error;

pub enum ClearTrajectoriesResponseType {
    BadRequest(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct ClearTrajectoriesPathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct ClearTrajectoriesQueryParameters {}

pub async fn clear_trajectories(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ClearTrajectoriesPathParameters,
) -> Result<ClearTrajectoriesResponseType, reqwest::Error> {
    let response = match client
        .delete(format!(
            "{}/cells/{}/controllers/{}/trajectories",
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
            Ok(error) => Ok(ClearTrajectoriesResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ClearTrajectoriesResponseType::UndefinedResponse(response)),
    }
}
