use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::list_trajectories_response::ListTrajectoriesResponse;

pub enum ListTrajectoriesResponseType {
    Ok(ListTrajectoriesResponse),

    UndefinedResponse(reqwest::Response),

    BadRequest(Error),
}

pub struct ListTrajectoriesPathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct ListTrajectoriesQueryParameters {}

pub async fn list_trajectories(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ListTrajectoriesPathParameters,
) -> Result<ListTrajectoriesResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
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
            Ok(error) => Ok(ListTrajectoriesResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<ListTrajectoriesResponse>().await {
            Ok(list_trajectories_response) => {
                Ok(ListTrajectoriesResponseType::Ok(list_trajectories_response))
            }
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListTrajectoriesResponseType::UndefinedResponse(response)),
    }
}
