use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::robot_tcps::RobotTcps;

pub enum ListVirtualControllerTcpsResponseType {
    Ok(RobotTcps),

    BadRequest(Error),

    UndefinedResponse(reqwest::Response),

    NotFound(Error),
}

pub struct ListVirtualControllerTcpsPathParameters {
    pub motion_group: String,

    pub controller: String,

    pub cell: String,
}

pub struct ListVirtualControllerTcpsQueryParameters {}

pub async fn list_virtual_controller_tcps(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ListVirtualControllerTcpsPathParameters,
) -> Result<ListVirtualControllerTcpsResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/virtual-controllers/{}/motion-groups/{}/tcps",
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
            Ok(error) => Ok(ListVirtualControllerTcpsResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<RobotTcps>().await {
            Ok(robot_tcps) => Ok(ListVirtualControllerTcpsResponseType::Ok(robot_tcps)),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(ListVirtualControllerTcpsResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListVirtualControllerTcpsResponseType::UndefinedResponse(
            response,
        )),
    }
}
