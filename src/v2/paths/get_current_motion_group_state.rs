use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::motion_group_state::MotionGroupState;

pub enum GetCurrentMotionGroupStateResponseType {
    Ok(MotionGroupState),

    BadRequest(Error),

    UndefinedResponse(reqwest::Response),

    NotFound(Error),
}

pub struct GetCurrentMotionGroupStatePathParameters {
    pub motion_group: String,

    pub cell: String,

    pub controller: String,
}

pub struct GetCurrentMotionGroupStateQueryParameters {
    pub response_coordinate_system: Option<String>,
}

pub async fn get_current_motion_group_state(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetCurrentMotionGroupStatePathParameters,

    query_parameters: GetCurrentMotionGroupStateQueryParameters,
) -> Result<GetCurrentMotionGroupStateResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.response_coordinate_system {
        reqwest_query_parameters.push(("response_coordinate_system", query_parameter.to_string()));
    }

    let response = match client
        .get(format!(
            "{}/cells/{}/controllers/{}/motion-groups/{}/state",
            server, path_parameters.cell, path_parameters.controller, path_parameters.motion_group
        ))
        .query(&reqwest_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        400 => match response.json::<Error>().await {
            Ok(error) => Ok(GetCurrentMotionGroupStateResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetCurrentMotionGroupStateResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<MotionGroupState>().await {
            Ok(motion_group_state) => Ok(GetCurrentMotionGroupStateResponseType::Ok(
                motion_group_state,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetCurrentMotionGroupStateResponseType::UndefinedResponse(
            response,
        )),
    }
}
