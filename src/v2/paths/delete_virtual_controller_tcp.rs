use ::reqwest;

use crate::v2::objects::error::Error;

pub enum DeleteVirtualControllerTcpResponseType {
    BadRequest(Error),

    NotFound(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct DeleteVirtualControllerTcpPathParameters {
    pub motion_group: String,

    pub tcp: String,

    pub cell: String,

    pub controller: String,
}

pub struct DeleteVirtualControllerTcpQueryParameters {}

pub async fn delete_virtual_controller_tcp(
    client: &reqwest::Client,

    server: &str,

    path_parameters: DeleteVirtualControllerTcpPathParameters,
) -> Result<DeleteVirtualControllerTcpResponseType, reqwest::Error> {
    let response = match client
        .delete(format!(
            "{}/cells/{}/virtual-controllers/{}/motion-groups/{}/tcps/{}",
            server,
            path_parameters.cell,
            path_parameters.controller,
            path_parameters.motion_group,
            path_parameters.tcp
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteVirtualControllerTcpResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteVirtualControllerTcpResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(DeleteVirtualControllerTcpResponseType::UndefinedResponse(
            response,
        )),
    }
}
