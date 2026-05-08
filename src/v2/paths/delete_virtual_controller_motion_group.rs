use ::reqwest;

use crate::v2::objects::error::Error;

pub enum DeleteVirtualControllerMotionGroupResponseType {
    NotFound(Error),

    UndefinedResponse(reqwest::Response),

    BadRequest(Error),
}

pub struct DeleteVirtualControllerMotionGroupPathParameters {
    pub motion_group: String,

    pub cell: String,

    pub controller: String,
}

pub struct DeleteVirtualControllerMotionGroupQueryParameters {}

pub async fn delete_virtual_controller_motion_group(
    client: &reqwest::Client,

    server: &str,

    path_parameters: DeleteVirtualControllerMotionGroupPathParameters,
) -> Result<DeleteVirtualControllerMotionGroupResponseType, reqwest::Error> {
    let response = match client
        .delete(format!(
            "{}/cells/{}/virtual-controllers/{}/motion-groups/{}",
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
            Ok(error) => Ok(DeleteVirtualControllerMotionGroupResponseType::NotFound(
                error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteVirtualControllerMotionGroupResponseType::BadRequest(
                error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(DeleteVirtualControllerMotionGroupResponseType::UndefinedResponse(response)),
    }
}
