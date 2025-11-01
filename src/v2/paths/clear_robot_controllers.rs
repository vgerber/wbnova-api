use ::reqwest;

use crate::v2::objects::error::Error;

pub enum ClearRobotControllersResponseType {
    NotFound(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct ClearRobotControllersPathParameters {
    pub cell: String,
}

pub struct ClearRobotControllersQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn clear_robot_controllers(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ClearRobotControllersPathParameters,

    query_parameters: ClearRobotControllersQueryParameters,
) -> Result<ClearRobotControllersResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        reqwest_query_parameters.push(("completion_timeout", query_parameter.to_string()));
    }

    let response = match client
        .delete(format!(
            "{}/cells/{}/controllers",
            server, path_parameters.cell
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
            Ok(error) => Ok(ClearRobotControllersResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ClearRobotControllersResponseType::UndefinedResponse(
            response,
        )),
    }
}
