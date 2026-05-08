use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::motion_group_joints::MotionGroupJoints;

pub enum SetMotionGroupStateResponseType {
    UndefinedResponse(reqwest::Response),

    BadRequest(Error),

    NotFound(Error),
}

pub struct SetMotionGroupStatePathParameters {
    pub motion_group: String,

    pub controller: String,

    pub cell: String,
}

pub struct SetMotionGroupStateQueryParameters {}

pub async fn set_motion_group_state(
    client: &reqwest::Client,

    server: &str,

    content: MotionGroupJoints,

    path_parameters: SetMotionGroupStatePathParameters,
) -> Result<SetMotionGroupStateResponseType, reqwest::Error> {
    let response = match client
        .put(format!(
            "{}/cells/{}/virtual-controllers/{}/motion-groups/{}/state",
            server, path_parameters.cell, path_parameters.controller, path_parameters.motion_group
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
            Ok(error) => Ok(SetMotionGroupStateResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(SetMotionGroupStateResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(SetMotionGroupStateResponseType::UndefinedResponse(response)),
    }
}
