use ::reqwest;

use crate::v2::objects::service_status_list::ServiceStatusList;

pub enum GetSystemStatusResponseType {
    UndefinedResponse(reqwest::Response),

    Ok(ServiceStatusList),
}

pub struct GetSystemStatusPathParameters {}

pub struct GetSystemStatusQueryParameters {}

pub async fn get_system_status(
    client: &reqwest::Client,

    server: &str,
) -> Result<GetSystemStatusResponseType, reqwest::Error> {
    let response = match client
        .get(format!("{}/system/status", server,))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<ServiceStatusList>().await {
            Ok(service_status_list) => Ok(GetSystemStatusResponseType::Ok(service_status_list)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetSystemStatusResponseType::UndefinedResponse(response)),
    }
}
