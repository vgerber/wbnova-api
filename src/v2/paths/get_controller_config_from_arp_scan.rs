use ::reqwest;

use crate::v2::objects::robot_controller::RobotController;

use crate::v2::objects::robot_controller_configuration_request::RobotControllerConfigurationRequest;

pub enum GetControllerConfigFromArpScanResponseType {
    UndefinedResponse(reqwest::Response),

    Ok(Vec<RobotController>),
}

pub struct GetControllerConfigFromArpScanPathParameters {}

pub struct GetControllerConfigFromArpScanQueryParameters {}

pub async fn get_controller_config_from_arp_scan(
    client: &reqwest::Client,

    server: &str,

    content: RobotControllerConfigurationRequest,
) -> Result<GetControllerConfigFromArpScanResponseType, reqwest::Error> {
    let response = match client
        .post(format!(
            "{}/experimental/system/network/controllers",
            server,
        ))
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<Vec<RobotController>>().await {
            Ok(robot_controllers) => Ok(GetControllerConfigFromArpScanResponseType::Ok(
                robot_controllers,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetControllerConfigFromArpScanResponseType::UndefinedResponse(response)),
    }
}
