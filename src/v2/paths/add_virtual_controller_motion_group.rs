use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::add_virtual_controller_motion_group_request::AddVirtualControllerMotionGroupRequest;

pub enum AddVirtualControllerMotionGroupResponseType {
    BadRequest(Error),

    NotFound(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct AddVirtualControllerMotionGroupPathParameters {
    pub controller: String,

    pub cell: String,
}

pub struct AddVirtualControllerMotionGroupQueryParameters {}

pub async fn add_virtual_controller_motion_group(
    client: &reqwest::Client,

    server: &str,

    content: AddVirtualControllerMotionGroupRequest,

    path_parameters: AddVirtualControllerMotionGroupPathParameters,
) -> Result<AddVirtualControllerMotionGroupResponseType, reqwest::Error> {
    let response = match client
        .post(format!(
            "{}/cells/{}/virtual-controllers/{}/motion-groups",
            server, path_parameters.cell, path_parameters.controller
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
            Ok(error) => Ok(AddVirtualControllerMotionGroupResponseType::BadRequest(
                error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(AddVirtualControllerMotionGroupResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(AddVirtualControllerMotionGroupResponseType::UndefinedResponse(response)),
    }
}
