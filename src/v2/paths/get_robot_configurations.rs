use ::reqwest;

pub enum GetRobotConfigurationsResponseType {
    UndefinedResponse(reqwest::Response),

    Ok(Vec<String>),
}

pub struct GetRobotConfigurationsPathParameters {}

pub struct GetRobotConfigurationsQueryParameters {}

pub async fn get_robot_configurations(
    client: &reqwest::Client,

    server: &str,
) -> Result<GetRobotConfigurationsResponseType, reqwest::Error> {
    let response = match client
        .get(format!("{}/robot-configurations", server,))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<Vec<String>>().await {
            Ok(robot_configurations) => {
                Ok(GetRobotConfigurationsResponseType::Ok(robot_configurations))
            }
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetRobotConfigurationsResponseType::UndefinedResponse(
            response,
        )),
    }
}
