use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::motion_group_description::MotionGroupDescription;

pub enum GetMotionGroupDescriptionResponseType {
    UndefinedResponse(reqwest::Response),

    NotFound(Error),

    Ok(MotionGroupDescription),

    BadRequest(Error),
}

pub struct GetMotionGroupDescriptionPathParameters {
    pub motion_group: String,

    pub controller: String,

    pub cell: String,
}

pub struct GetMotionGroupDescriptionQueryParameters {}

pub async fn get_motion_group_description(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetMotionGroupDescriptionPathParameters,
) -> Result<GetMotionGroupDescriptionResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/controllers/{}/motion-groups/{}/description",
            server, path_parameters.cell, path_parameters.controller, path_parameters.motion_group
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetMotionGroupDescriptionResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<MotionGroupDescription>().await {
            Ok(motion_group_description) => Ok(GetMotionGroupDescriptionResponseType::Ok(
                motion_group_description,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(GetMotionGroupDescriptionResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetMotionGroupDescriptionResponseType::UndefinedResponse(
            response,
        )),
    }
}
