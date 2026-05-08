use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::motion_group_infos::MotionGroupInfos;

pub enum GetMotionGroupsResponseType {
    UndefinedResponse(reqwest::Response),

    NotFound(Error),

    BadRequest(Error),

    Ok(MotionGroupInfos),
}

pub struct GetMotionGroupsPathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct GetMotionGroupsQueryParameters {}

pub async fn get_motion_groups(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetMotionGroupsPathParameters,
) -> Result<GetMotionGroupsResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/virtual-controllers/{}/motion-groups",
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
            Ok(error) => Ok(GetMotionGroupsResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<MotionGroupInfos>().await {
            Ok(motion_group_infos) => Ok(GetMotionGroupsResponseType::Ok(motion_group_infos)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetMotionGroupsResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetMotionGroupsResponseType::UndefinedResponse(response)),
    }
}
