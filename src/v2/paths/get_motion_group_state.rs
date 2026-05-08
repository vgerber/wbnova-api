use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::motion_group_joints::MotionGroupJoints;

pub enum GetMotionGroupStateResponseType {
    BadRequest(Error),

    Ok(MotionGroupJoints),

    UndefinedResponse(reqwest::Response),

    NotFound(Error),
}

pub struct GetMotionGroupStatePathParameters {
    pub cell: String,

    pub controller: String,

    pub motion_group: String,
}

pub struct GetMotionGroupStateQueryParameters {}

pub async fn get_motion_group_state(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetMotionGroupStatePathParameters,
) -> Result<GetMotionGroupStateResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/virtual-controllers/{}/motion-groups/{}/state",
            server, path_parameters.cell, path_parameters.controller, path_parameters.motion_group
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        400 => match response.json::<Error>().await {
            Ok(error) => Ok(GetMotionGroupStateResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<MotionGroupJoints>().await {
            Ok(motion_group_joints) => Ok(GetMotionGroupStateResponseType::Ok(motion_group_joints)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetMotionGroupStateResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetMotionGroupStateResponseType::UndefinedResponse(response)),
    }
}
