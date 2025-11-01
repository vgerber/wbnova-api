use ::reqwest;

use crate::v2::objects::name_list::NameList;

pub enum ListRobotControllersResponseType {
    Ok(NameList),

    UndefinedResponse(reqwest::Response),
}

pub struct ListRobotControllersPathParameters {
    pub cell: String,
}

pub struct ListRobotControllersQueryParameters {}

pub async fn list_robot_controllers(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ListRobotControllersPathParameters,
) -> Result<ListRobotControllersResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/controllers",
            server, path_parameters.cell
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<NameList>().await {
            Ok(name_list) => Ok(ListRobotControllersResponseType::Ok(name_list)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListRobotControllersResponseType::UndefinedResponse(
            response,
        )),
    }
}
