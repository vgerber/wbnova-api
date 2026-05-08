use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::robot_tcp_data::RobotTcpData;

pub enum AddVirtualControllerTcpResponseType {
    BadRequest(Error),

    NotFound(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct AddVirtualControllerTcpPathParameters {
    pub controller: String,

    pub cell: String,

    pub tcp: String,

    pub motion_group: String,
}

pub struct AddVirtualControllerTcpQueryParameters {}

pub async fn add_virtual_controller_tcp(
    client: &reqwest::Client,

    server: &str,

    content: RobotTcpData,

    path_parameters: AddVirtualControllerTcpPathParameters,
) -> Result<AddVirtualControllerTcpResponseType, reqwest::Error> {
    let response = match client
        .put(format!(
            "{}/cells/{}/virtual-controllers/{}/motion-groups/{}/tcps/{}",
            server,
            path_parameters.cell,
            path_parameters.controller,
            path_parameters.motion_group,
            path_parameters.tcp
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
            Ok(error) => Ok(AddVirtualControllerTcpResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(AddVirtualControllerTcpResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(AddVirtualControllerTcpResponseType::UndefinedResponse(
            response,
        )),
    }
}
